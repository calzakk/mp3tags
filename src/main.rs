use std::env::set_current_dir;
use std::fs::{read_dir, read_to_string};
use std::io;
use std::io::Write;
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Only check the script, do not execute it
    #[arg(short, long, default_value_t = false)]
    check: bool,

    /// Create a script for the files in the current directory
    #[arg(short, long, default_value_t = false)]
    new: bool,

    /// Script pathname
    #[arg()]
    script: String
}

fn main() {
    let args = Args::parse();

    let script_pathname = Path::new(&args.script);

    set_current_directory(script_pathname);

    if args.new {
        new_script(script_pathname);
        return;
    }

    let script = read_file(script_pathname);
    println!("Script read");

    let tracks = parse_script(script);
    println!("Script parsed, {} tracks", tracks.iter().filter(|track| !track.is_default).count());

    validate_script(&tracks);
    println!("Script validated");

    if !args.check {
        process_script(&tracks);
        println!("Script processed, no errors");
    }
}

#[derive(Debug, Clone)]
struct Track {
    is_default: bool,
    original_filename: String,
    new_filename: String,
    track_number: String,
    title: String,
    artist: String,
    album: String,
    year: String,
    genre: String,
    delete_tags: Vec<String>,
}

impl Track {
    fn new(is_default: bool) -> Track {
        Track {
            is_default,
            original_filename: String::new(),
            new_filename: String::new(),
            track_number: String::new(),
            title: String::new(),
            artist: String::new(),
            album: String::new(),
            year: String::new(),
            genre: String::new(),
            delete_tags: Vec::new(),
        }
    }
}

fn set_current_directory(script_pathname: &Path) {
    let parent = script_pathname.parent().unwrap();
    if parent.is_dir() {
        set_current_dir(parent).expect(format!("Unable to change directory to {}", parent.to_str().unwrap()).as_str());
    }
}

fn new_script(script_pathname: &Path) {
    let files = read_dir(".").unwrap();

    let create_error = "Unable to create file";
    let write_error = "Unable to write to file";

    let mut script_file = std::fs::File::create(script_pathname).expect(create_error);

    writeln!(script_file, "new_filename={{track}} {{title}}.mp3").expect(write_error);
    writeln!(script_file, "artist=TODO").expect(write_error);
    writeln!(script_file, "album=TODO").expect(write_error);
    writeln!(script_file, "year=TODO").expect(write_error);
    writeln!(script_file, "genre=TODO").expect(write_error);
    writeln!(script_file, "delete_tag=TXXX").expect(write_error);
    writeln!(script_file, "delete_tag=TPE2").expect(write_error);
    writeln!(script_file, "delete_tag=TSSE").expect(write_error);

    for file in files {
        let filename = file.unwrap().file_name();
        if !filename.to_str().unwrap().ends_with(".mp3") {
            continue;
        }
        writeln!(script_file).expect(write_error);
        writeln!(script_file, "file={}", filename.to_str().unwrap()).expect(write_error);
        writeln!(script_file, "track=TODO").expect(write_error);
        writeln!(script_file, "title=TODO").expect(write_error);
    }

    println!("Script created");
}

fn read_file(pathname: &Path) -> String {
    read_to_string(pathname).expect("Unable to read file")
}

fn parse_script(script: String) -> Vec<Track> {
    let mut track = Track::new(true);
    let mut tracks = Vec::new();

    let lines: Vec<&str> = script.lines().collect();
    let mut line_number = 0;

    for line in lines {
        line_number += 1;

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("#") {
            continue;
        }

        if line.starts_with("file=") {
            tracks.push(track);
            track = Track::new(false);
            track.original_filename = trimmed[5..].to_string();
        } else if line.starts_with("new_filename=") {
            track.new_filename = trimmed[13..].to_string();
        } else if line.starts_with("artist=") {
            track.artist = trimmed[7..].to_string();
        } else if line.starts_with("track=") {
            track.track_number = trimmed[6..].to_string();
        } else if line.starts_with("title=") {
            track.title = trimmed[6..].to_string();
        } else if line.starts_with("album=") {
            track.album = trimmed[6..].to_string();
        } else if line.starts_with("year=") {
            track.year = trimmed[5..].to_string();
        } else if line.starts_with("genre=") {
            track.genre = trimmed[6..].to_string();
        } else if line.starts_with("delete_tag=") {
            track.delete_tags.push(trimmed[11..].to_string());
        } else {
            panic!("Error on line {}: {}", line_number, line);
        }
    }
    tracks.push(track);

    tracks
}

fn validate_script(tracks: &Vec<Track>) {
    let mut track_number = 0;
    for track in tracks {
        if track.is_default {
            continue;
        }

        track_number += 1;
        if track.original_filename.is_empty() {
            panic!("Track {} has no filename", track_number);
        }
        if !Path::new(&track.original_filename).try_exists().unwrap() {
            panic!("Track {} does not exist", track.original_filename);
        }
    }
}

fn process_script(tracks: &Vec<Track>) {
    let defaults = if tracks[0].is_default {
        tracks[0].clone()
    } else {
        Track::new(true)
    };

    for track in tracks {
        if track.is_default {
            continue;
        }

        println!("{}", track.original_filename);
        edit_tags(&track, &defaults);
        delete_tags(&track, &defaults);
        rename_file(&track, &defaults);
    }
}

fn edit_tags(track: &Track, defaults: &Track) {
    let args = vec![
        "--track-number", &track.track_number,
        "--title", &track.title,
        "--artist", get_value_or_default(&track.artist, &defaults.artist),
        "--album", get_value_or_default(&track.album, &defaults.album),
        "--year", get_value_or_default(&track.year, &defaults.year),
        "--genre", get_value_or_default(&track.genre, &defaults.genre),
        &track.original_filename
    ];
    execute_command("editag", args);
}

fn delete_tags(track: &Track, defaults: &Track) {
    for tag in get_vector_value_or_default(&track.delete_tags, &defaults.delete_tags) {
        let args = vec![
            "--delete-tag", &tag,
            &track.original_filename
        ];
        execute_command("editag", args);
    }
}

fn rename_file(track: &Track, defaults: &Track) {
    let new_filename = if !track.new_filename.is_empty() {
        &track.new_filename
    } else if !defaults.new_filename.is_empty() {
        &defaults.new_filename
    } else {
        return
    };

    let new_filename = replacements(&new_filename, &track, &defaults);
    println!("=> {}", new_filename);

    let args = vec![
        &track.original_filename,
        new_filename.as_str()
    ];
    execute_command("mv", args);
}

fn execute_command(command: &str, args: Vec<&str>) {
    let output = std::process::Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    if !output.status.success() {
        panic!("Error: exit code {}", output.status.code().unwrap());
    }
}

fn replacements(filename: &String, track: &Track, defaults: &Track) -> String {
    filename.replace("{track}", &track.track_number)
        .replace("{title}", &track.title)
        .replace("{artist}", get_value_or_default(&track.artist, &defaults.artist))
        .replace("{album}", get_value_or_default(&track.album, &defaults.album))
        .replace("{year}", get_value_or_default(&track.year, &defaults.year))
        .replace("{genre}", get_value_or_default(&track.genre, &defaults.genre))
}

fn get_value_or_default<'a>(value: &'a str, default: &'a str) -> &'a str {
    if value.is_empty() {
        default
    } else {
        value
    }
}

fn get_vector_value_or_default<'a>(value: &'a Vec<String>, default: &'a Vec<String>) -> &'a Vec<String> {
    if value.is_empty() {
        default
    } else {
        value
    }
}