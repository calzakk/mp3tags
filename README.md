# mp3tags

Edit tags for mp3 files, using a script file.

Usage:

    mp3tags [-n] [-c] <script>

Ensure [editag](https://github.com/blackout358/editag) is installed and accessible via the PATH.

Use `-n` to create a new script for the mp3 files in the directory:

    mp3tags -n script

Update the values in the script.

Use `-c` to validate the script, but not execute it:

    mp3tags -c script

Lastly, execute the script:

    mp3tags script

## To Do

- Improved generation of scripts: auto-detect track numbers and titles.

## Licence

The MIT License

Copyright (c) 2024 Graham Bull

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
