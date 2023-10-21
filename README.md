# mingrep

mingrep is a simple clone of the `grep` command-line tool, written in Rust. It allows you to search for a specific string in a text file, search specific string in all directory files with the option to perform a case-insensitive search.

## Usage

To use mingrep, you can run it from the command line with the following syntax:

## mingrep 

cargo run -- [query] [file path or directory path] 

if you search for a specific string in one file, you can run with or without case sensitive:

IGNORE_CASE=0 cargo run -- [query] [file path] 

- `--ignore-case` (optional): Perform a case-insensitive search.
- `<query>`: The string you want to search for in the file.
- `<file_path / directory_path>`: The path to the file in which you want to search.


## Examples

### Search for the string "how" in a file named `sample.txt`:

IGNORE_CASE=0 cargo run -- "how" "/folder/folder/folder/folder/folder/folder/text.txt"
    

### Search for string "println" in all files with rs extension:

cargo run -- "println" "/folder/folder/folder/folder/folder/folder/*.rs"

    