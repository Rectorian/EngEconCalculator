fn main() {
    EngEconCalculator::test_ansi();
    use EngEconCalculator::ansi_commands;

    //print!("{}", ansi_commands::ANSI_CLEAR);
    println!(
        "{}",
        ansi_commands::disp_text_colored(String::from("This is a test"), 13)
    );
}
