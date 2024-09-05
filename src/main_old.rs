fn main() {
    EngEconCalculator::test_ansi();
    use EngEconCalculator::ansi_commands;

    //print!("{}", ansi_commands::ANSI_CLEAR);
    println!(
        "{}\n{}",
        ansi_commands::get_text_colored(String::from("This is a test"), 13),
        ansi_commands::get_text_colored(
            String::from("This text should be a different color on a new line"),
            214
        ),
    );

    print!("\x1B[1000:1000H");
    print!("\x1B[6n");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    //ansi_commands::sleep_5();
    println!("Input read as {:?}", input);
    //println!("This is a test");
}
