use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Get the current directory or the directory specified as a command-line argument
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    // Read the directory entries
    let entries = fs::read_dir(path)?;

    // Iterate over the entries and print the file names
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        println!("{}", file_name);
    }

    Ok(())
}
