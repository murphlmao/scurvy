fn main() {
    let my_message = String::from("Hello, Rust!");
    scurvy::functionality_wrapper::display_message("Static message");
    scurvy::functionality_wrapper::display_message(&my_message);

}
