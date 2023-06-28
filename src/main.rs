use glob::glob;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: ./program <glob_pattern> <last_digit>");
        return;
    }

    let glob_pattern = &args[1];
    let last_digit: u32 = match args[2].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid last digit");
            return;
        }
    };

    // Get a list of file names matching the glob pattern and the last digit of the hash
    let files = get_files_with_matching_last_digit(glob_pattern, last_digit);

    // Print the file names
    if files.is_empty() {
        println!(
            "No files found with hash matching the last digit {}",
            last_digit
        );
    } else {
        for file in files {
            println!("{}", file);
        }
    }
}

fn get_files_with_matching_last_digit(glob_pattern: &str, last_digit: u32) -> Vec<String> {
    let mut files_matching_last_digit = Vec::new();

    // Glob the file paths
    let paths = match glob(glob_pattern) {
        Ok(paths) => paths,
        Err(_) => {
            println!("Invalid glob pattern");
            return files_matching_last_digit;
        }
    };

    // Process each path
    for path in paths {
        if let Ok(path) = path {
            if path.is_file() {
                let mut hasher = DefaultHasher::new();
                path.hash(&mut hasher);
                let hash = hasher.finish();
                let last_hash_digit = hash % 10;

                if last_hash_digit == last_digit as u64 {
                    // print path
                    let path = path.to_str().unwrap();
                    files_matching_last_digit.push(path.to_string());
                }
            }
        }
    }

    files_matching_last_digit
}
