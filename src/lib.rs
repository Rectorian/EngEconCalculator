pub mod invest_items;

pub mod ansi_commands {
    const ANSI_CLEAR: &str = "\x1B[3J\x1B[2J\x1B[H";
    const ANSI_RESET_COL: &str = "\x1B[39m";
    const RS_ON: bool = false;

    //Function to get colored text and reset it.
    pub fn get_text_colored(text: &str, color_val: u8) -> String {
        let set_color_sequence: String = format!("\x1B[38;5;{}m", color_val);
        format!("{}{}{}", set_color_sequence, text, ANSI_RESET_COL)
    }

    pub fn clear_screen(respect_reset: bool) {
        if !respect_reset {
            print!("{ANSI_CLEAR}")
        } else if RS_ON {
            print!("{ANSI_CLEAR}");
        }
    }

    pub fn disable_cursor() {
        print!("\x1B[?;25;l");
    }

    pub fn enable_cursor() {
        print!("\x1B[?;25;h");
    }
}

pub mod user_interface {
    pub fn grab_user_input(text_prompt: &str) -> String {
        let mut output = String::new();

        println!("{}", text_prompt);

        std::io::stdin()
            .read_line(&mut output)
            .expect("Invalid Input");

        output
    }

    pub fn grab_user_num(text_prompt: &str) -> f64 {
        let mut input = grab_user_input(text_prompt);
        let out_num: f64;

        loop {
            out_num = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a valid number input");
                    input = grab_user_input(text_prompt);
                    continue;
                }
            };

            break;
        }

        out_num
    }

    pub fn grab_user_num_restricted(text_prompt: &str, low: f64, high: f64) -> f64 {
        let mut out_num: f64;

        while {
            out_num = grab_user_num(text_prompt);

            (low <= out_num) && (out_num <= high)
        } {
            println!("Number is not within input range [{low}:{high}]");
        }

        out_num
    }

    pub fn user_f_from_p() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let principal = data::AmountType::Principal(data::Amount::Fl64(grab_user_num(
            "Enter Principal Amount:",
        )));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));
        cs(true);

        let result = match investment_calculations::calculations::f_from_p(
            &principal, &periods, &interest,
        ) {
            Ok(num) => {
                let out_num_format = format!("${:.2}", num);
                gtc(out_num_format.as_str(), 10)
            }
            Err(err_mes) => gtc(err_mes.as_str(), 9),
        };

        println!("Final amount calculated as: {}", result);
    }

    pub fn user_p_from_f() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let final_ =
            data::AmountType::Final(data::Amount::Fl64(grab_user_num("Enter Final Amount:")));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result =
            match investment_calculations::calculations::p_from_f(&final_, &periods, &interest) {
                Ok(num) => {
                    let out_num_format = format!("${:.2}", num);
                    gtc(out_num_format.as_str(), 10)
                }
                Err(err_mes) => gtc(err_mes.as_str(), 9),
            };

        println!("Principal amount calculated as: {}", result);
    }

    pub fn user_p_from_a() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let uniform =
            data::AmountType::Uniform(data::Amount::Fl64(grab_user_num("Enter Uniform Amount:")));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result =
            match investment_calculations::calculations::p_from_a(&uniform, &periods, &interest) {
                Ok(num) => {
                    let out_num_format = format!("${:.2}", num);
                    gtc(out_num_format.as_str(), 10)
                }
                Err(err_mes) => gtc(err_mes.as_str(), 9),
            };

        println!("Minimum Principal for {}: {}", uniform, result);
    }

    pub fn user_a_from_p() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let principal = data::AmountType::Principal(data::Amount::Fl64(grab_user_num(
            "Enter Principal Amount:",
        )));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result = match investment_calculations::calculations::a_from_p(
            &principal, &periods, &interest,
        ) {
            Ok(num) => {
                let out_num_format = format!("${:.2}", num);
                gtc(out_num_format.as_str(), 10)
            }
            Err(err_mes) => gtc(err_mes.as_str(), 9),
        };

        println!("Maximum Withdrawal Amount for {}: {}", principal, result);
    }

    pub fn user_f_from_a() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let uniform =
            data::AmountType::Uniform(data::Amount::Fl64(grab_user_num("Enter Uniform Amount:")));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result =
            match investment_calculations::calculations::f_from_a(&uniform, &periods, &interest) {
                Ok(num) => {
                    let out_num_format = format!("${:.2}", num);
                    gtc(out_num_format.as_str(), 10)
                }
                Err(err_mes) => gtc(err_mes.as_str(), 9),
            };

        println!("Maximum Final Amount for {}: {}", uniform, result);
    }

    pub fn user_a_from_f() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let final_ =
            data::AmountType::Final(data::Amount::Fl64(grab_user_num("Enter Final Amount:")));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result =
            match investment_calculations::calculations::a_from_f(&final_, &periods, &interest) {
                Ok(num) => {
                    let out_num_format = format!("${:.2}", num);
                    gtc(out_num_format.as_str(), 10)
                }
                Err(err_mes) => gtc(err_mes.as_str(), 9),
            };

        println!("Minimum Payments Needed for {}: {}", final_, result);
    }

    pub fn user_p_from_g() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let gradient =
            data::AmountType::Gradient(data::Amount::Fl64(grab_user_num("Enter Gradient Amount:")));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result =
            match investment_calculations::calculations::p_from_g(&gradient, &periods, &interest) {
                Ok(num) => {
                    let out_num_format = format!("${:.2}", num);
                    gtc(out_num_format.as_str(), 10)
                }
                Err(err_mes) => gtc(err_mes.as_str(), 9),
            };

        println!("Present Worth of {}: {}", gradient, result);
    }

    pub fn user_a_from_g() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let gradient =
            data::AmountType::Gradient(data::Amount::Fl64(grab_user_num("Enter Gradient Amount:")));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result =
            match investment_calculations::calculations::a_from_g(&gradient, &periods, &interest) {
                Ok(num) => {
                    let out_num_format = format!("${:.2}", num);
                    gtc(out_num_format.as_str(), 10)
                }
                Err(err_mes) => gtc(err_mes.as_str(), 9),
            };

        println!("Equivalent Uniform Payment for {}: {}", gradient, result);
    }

    pub fn user_f_from_g() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let gradient =
            data::AmountType::Gradient(data::Amount::Fl64(grab_user_num("Enter Gradient Amount:")));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result =
            match investment_calculations::calculations::f_from_g(&gradient, &periods, &interest) {
                Ok(num) => {
                    let out_num_format = format!("${:.2}", num);
                    gtc(out_num_format.as_str(), 10)
                }
                Err(err_mes) => gtc(err_mes.as_str(), 9),
            };

        println!("Future Worth of {}: {}", gradient, result);
    }

    pub fn user_p_from_g_rate() {
        use super::{
            ansi_commands::{clear_screen as cs, get_text_colored as gtc},
            invest_items::{data, investment_calculations},
        };

        cs(true);

        let gradient = data::AmountType::GradientRate(data::InterestType::Compound(grab_user_num(
            "Enter Gradient Rate as Decimal:",
        )));

        let initial_payment = data::AmountType::Uniform(data::Amount::Fl64(grab_user_num(
            "Enter the Initial Payment Amount:",
        )));

        let interest = data::AmountType::InterestRate(data::InterestType::Compound(grab_user_num(
            "Enter Interest Rate as Decimal:",
        )));

        let periods = data::AmountType::TimePeriods(data::Amount::In32(grab_user_num(
            "Enter Number of Compounding Periods:",
        ) as i32));

        cs(true);

        let result = match investment_calculations::calculations::p_from_g_rate(
            &initial_payment,
            &gradient,
            &periods,
            &interest,
        ) {
            Ok(num) => {
                let out_num_format = format!("${:.2}", num);
                gtc(out_num_format.as_str(), 10)
            }
            Err(err_mes) => gtc(err_mes.as_str(), 9),
        };

        println!("Future Worth of {}: {}", gradient, result);
    }
}
