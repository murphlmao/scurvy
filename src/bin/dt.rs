// print directory tree

use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut path = "";
    let mut excludes = Vec::<String>::new();
    let mut max_depth: Option<usize> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-e" if i + 1 < args.len() => {
                let excludes_arg = &args[i + 1];
                println!("{}", excludes_arg);
                let trimmed = excludes_arg.trim_start_matches('"').trim_end_matches('"');
                println!("{}", excludes_arg);
                excludes = parse_excludes(trimmed);
                i += 1; // Skip the next argument because it's part of -e
            },
            _ if path.is_empty() => {
                path = &args[i];
            },
            _ => {
                if max_depth.is_none() && args[i].parse::<usize>().is_ok() {
                    // Try parsing the depth, ignoring non-numeric arguments
                    max_depth = args[i].parse::<usize>().ok();
                }
            }
        }
        i += 1;
    }

    scurvy::functionality_wrapper::directory_tree(path, max_depth, 0, excludes);
    Ok(())
}

fn parse_excludes(excludes_str: &str) -> Vec<String> {
    excludes_str.split(',')
                .map(|s| s.trim().trim_matches('"'))
                .map(ToString::to_string)
                .collect()
}
