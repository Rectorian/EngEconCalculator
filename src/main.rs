fn main() {
    EngEconCalculator::ansi_commands::clear_screen();

    println!(
        "{}",
        EngEconCalculator::ansi_commands::get_text_colored(
            String::from("Welcome to the Calculator"),
            6
        )
    );
}
