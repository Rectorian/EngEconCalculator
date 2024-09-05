fn main() {
    print!("{}", EngEconCalculator::ansi_commands::ANSI_CLEAR);
    println!(
        "{}",
        EngEconCalculator::ansi_commands::get_text_colored(
            String::from("Welcome to the Calculator"),
            6
        )
    );
}
