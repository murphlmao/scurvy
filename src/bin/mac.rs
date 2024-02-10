fn main() {
    let my_message = String::from("Hello, Rust!");
    scurvy::utils::print_mac_address("boobies");
    scurvy::utils::display_message("Static message");
    scurvy::utils::display_message(&my_message);

}
