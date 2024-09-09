fn main() {
    EngEconCalculator::ansi_commands::clear_screen(false);
    use EngEconCalculator::user_interface::*;

    //user_f_from_p();
    //user_p_from_f();
    //user_a_from_p();
    user_p_from_a();
    //user_a_from_f();
    //user_f_from_a();
}

fn run() {
    EngEconCalculator::ansi_commands::clear_screen(false);
    use EngEconCalculator::user_interface as UI;

    let mut user_input = String::new();

    while {
        //Simple do part of the loop to grab the user input. Then checks for exit and returns
        //corresponding conditional
        user_input = UI::grab_user_input("What would you like to calculate? (Type ? for options)");
        match user_input.to_lowercase().as_str() {
            "e" | "end" | "exit" => false,
            _ => true,
        }
    } {
        match user_input.to_lowercase().as_str() {
            "?" => {
                println!("Options are:");
                println!("\tExit (E, End, Exit)");
                println!("\tPresent Value (P, Present)");
                println!("\tFuture Value (F, Future)");
                println!("\tUniform Value (A, U, Uniform)");
                println!("\tArithmetic Gradient (AG, Arithmetic)");
                println!("\tPresent Value Equivalent of Geometric Gradient (PG, Geometric)");
            }
            "p" | "present" => {
                //TODO Put Function Here
            }
            "f" | "future" => {
                //TODO Put Function Here
            }
            "a" | "u" | "uniform" => {
                //TODO Put function here
            }
            "ag" | "arithmetic" => {
                //TODO Put Function Here
            }
            "pg" | "geometric" => {
                //TODO Put Function Here
            }
            _ => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Invalid Input. Please Try Again");
                continue;
            }
        }
    }
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
