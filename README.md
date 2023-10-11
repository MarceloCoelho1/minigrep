# mingrep

mingrep is a simple clone of the `grep` command-line tool, written in Rust. It allows you to search for a specific string in a text file, with the option to perform a case-insensitive search.

## Usage

To use mingrep, you can run it from the command line with the following syntax:

## mingrep [--ignore-case] <query> <file_path> 

- `--ignore-case` (optional): Perform a case-insensitive search.
- `<query>`: The string you want to search for in the file.
- `<file_path>`: The path to the file in which you want to search.

**not allowed space in string**

## Examples

1. Search for the string "rust" in a file named `sample.txt`:

IGNORE_CASE=0 cargo run -- how /home/marcelo/Desktop/side-projects/mingrep/src/text.txt
    
return:

6: How dreary to be somebody! 
7: How public, like a frog  
12: How can i do it 
    