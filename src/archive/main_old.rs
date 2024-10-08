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

fn _test_1() {
    EngEconCalculator::ansi_commands::clear_screen(false);
    use EngEconCalculator::ansi_commands::get_text_colored as gtc;

    println!(
        "{}",
        EngEconCalculator::ansi_commands::get_text_colored(
            "This is a test of the principal calculation:",
            13
        )
    );

    let result = {
        use EngEconCalculator::invest_items::data;
        use EngEconCalculator::invest_items::investment_calculations::calculations;

        let final_amt = data::AmountType::Final(data::Amount::In64(100000));
        let period = data::AmountType::TimePeriods(data::Amount::In32(5));
        let rate = data::AmountType::InterestRate(data::InterestType::Compound(0.10));

        match calculations::p_from_f(&final_amt, &period, &rate) {
            Ok(num) => num,
            Err(error_message) => {
                println!("Error: {}", error_message);
                0.0
            }
        }
    };

    let result = format!("${:.2}", result);

    println!(
        "Result for period of 5 years, rate of 10% compound, and final amount of $100k: {}",
        gtc(result.as_str(), 10)
    );

    let result = {
        use EngEconCalculator::invest_items::data;
        use EngEconCalculator::invest_items::investment_calculations::calculations;

        let final_amt = data::AmountType::Principal(data::Amount::In64(100000));
        let period = data::AmountType::TimePeriods(data::Amount::In32(5));
        let rate = data::AmountType::InterestRate(data::InterestType::Simple(0.10));

        match calculations::f_from_p(&final_amt, &period, &rate) {
            Ok(num) => num,
            Err(error_message) => {
                println!("Error: {}", error_message);
                0.0
            }
        }
    };

    let result = format!("${:.2}", result);

    println!(
        "Result for period of 5 years, rate of 10% compounding, and principal amount of $100k: {}",
        gtc(result.as_str(), 10)
    );

    match EngEconCalculator::invest_items::investment_calculations::unit_tests::unit_test(
        "exponential",
    ) {
        Ok(out) => println!("{}", out),
        Err(err_mes) => println!("{}", err_mes),
    }

    match EngEconCalculator::invest_items::investment_calculations::unit_tests::unit_test(
        "uniform Payments",
    ) {
        Ok(out) => println!("{}", out),
        Err(err_mes) => println!("{}", err_mes),
    }
}
