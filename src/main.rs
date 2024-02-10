use std::env;
use scurvy::utils; // Adjust `your_project` to the name of your crate

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 {
        args[1].clone() // Use the first argument if provided
    } else {
        // Derive the command from the executable name
        env::current_exe()
            .ok()
            .and_then(|path| path.file_stem()
                .map(|name| name.to_str().unwrap_or("").to_lowercase()))
            .unwrap_or_else(|| "unknown".to_string())
    };

    match command.as_str() {
        //"mac" | "mac.exe" => utils::print_mac_address(),
        "ip" | "ip.exe" => utils::print_ip_address(),
        _ => println!("Unknown command or executable name."),
    }
}