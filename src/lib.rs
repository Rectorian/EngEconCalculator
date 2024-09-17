pub mod invest_items;

pub mod ansi_commands {
    const ANSI_CLEAR: &str = "\x1B[3J\x1B[2J\x1B[H";
    const ANSI_RESET_COL: &str = "\x1B[39m";
    const RS_ON: bool = true;

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
    use crate::invest_items::data::WIT;

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

    //Function used get any of the amount types from the user.
    pub fn get_user_amount_type(
        text_prompt: &str,
    ) -> Option<super::invest_items::data::AmountType> {
        use super::{ansi_commands, invest_items::data};

        loop {
            let input = grab_user_input(text_prompt);

            let input = input.trim().to_lowercase();

            match input.as_str() {
                "p" | "principal" => {
                    return Some(data::AmountType::Principal(data::Amount::Fl64(
                        grab_user_num("Enter Principal Amount:"),
                    )));
                }
                "f" | "final" => {
                    return Some(data::AmountType::Final(data::Amount::Fl64(grab_user_num(
                        "Enter Final Amount:",
                    ))));
                }
                "a" | "u" | "uniform" => {
                    return Some(data::AmountType::Uniform(data::Amount::Fl64(
                        grab_user_num("Enter Uniform Amount:"),
                    )));
                }
                "g" | "arithmetic" => {
                    return Some(data::AmountType::Gradient(data::Amount::Fl64(
                        grab_user_num("Enter Arithmetic Gradient Amount:"),
                    )));
                }
                "gr" | "geometric" => {
                    return Some(data::AmountType::GradientRate(
                        data::InterestType::Compound(grab_user_num(
                            "Enter Gradient Rate as Decimal:",
                        )),
                    ));
                }
                "t" | "periods" => {
                    return Some(data::AmountType::TimePeriods(data::Amount::Fl64(
                        grab_user_num("Enter Number of Periods:"),
                    )));
                }
                "i" | "interest" => {
                    return Some(data::AmountType::InterestRate(
                        data::InterestType::Compound(grab_user_num("Enter Principal Amount:")),
                    ));
                }
                "?" => {
                    ansi_commands::clear_screen(true);
                    println!("Possible inputs are:");
                    println!("\tPrincipal (P, Principal)");
                    println!("\tFinal (F, Final)");
                    println!("\tUniform (A, U, Uniform)");
                    println!("\tArithmetic Gradient (G, Arithmetic)");
                    println!("\tGeometric Gradient (GR, Geometric)");
                    println!("\tInterest Rate (I, Interest)");
                    continue;
                }
                _ => {
                    println!("Invalid option. Please re-enter (type '?' for list of options)");
                    continue;
                }
            }
        }
    }

    pub fn get_user_flow_type() -> super::invest_items::cli_disp::FlowType {
        use super::invest_items::cli_disp::FlowType;

        loop {
            let input = grab_user_input("Enter flow type:");
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "p" | "payment" => {
                    return FlowType::Payment;
                }
                "w" | "withdrawal" => {
                    return FlowType::Withdrawal;
                }
                "?" => {
                    super::ansi_commands::clear_screen(true);
                    println!("Possible inputs are:");
                    println!("\tPayment (P, Payment)");
                    println!("\tWithdrawal (W, Withdrawal)");
                    continue;
                }
                _ => {
                    println!("Invalid option. Please re-enter (type '?' for list of options)");
                    continue;
                }
            }
        }
    }

    pub fn get_user_time_type() -> super::invest_items::cli_disp::TimeType {
        use super::invest_items::cli_disp::TimeType;

        let time_type: String;
        loop {
            let input = grab_user_input("What is payment time type:");

            let input = input.trim().to_lowercase();

            time_type = match input.as_str() {
                "s" | "single" => String::from("single"),
                "m" | "multi" => String::from("multi"),
                "r" | "range" => String::from("range"),
                "?" => {
                    super::ansi_commands::clear_screen(true);
                    println!("Possible inputs are:");
                    println!("\tSingle Flow (S, Single)");
                    println!("\tMultiple Non-Uniform Flows (M, Multi)");
                    println!("\tRange (uniform) of Time Flow Occurs (R, Range)");
                    continue;
                }
                _ => {
                    println!("Invalid option. Please re-enter (type '?' for list of options)");
                    continue;
                }
            };

            break;
        }

        let out = match time_type.as_str() {
            "single" => {
                TimeType::Single(grab_user_num("Enter Time when Single Flow Occurs:") as i32)
            }
            "multi" => {
                let num_times = grab_user_num("How Many Times does Flow Occur:") as i32;

                let mut times: Vec<i32> = vec![];

                for num in 1..=num_times {
                    times.push(grab_user_num(format!("Flow #{num} Occurs:").as_str()) as i32);
                }

                TimeType::Multi(times)
            }
            "range" => {
                let first = grab_user_num("What is the Fist Time Flow Occurs:") as i32;
                let second = grab_user_num("What is the Second Time Flow Occurs:") as i32;

                TimeType::Range((first, second))
            }
            _ => panic!("Something went horribly wrong with our get_user_time_type function"),
        };

        out
    }

    pub fn build_user_cash_flow() -> super::invest_items::cli_disp::CashFlow {
        let name = grab_user_input("What would you like to label this cashflow as:");

        let amount: super::invest_items::data::AmountType;

        loop {
            match get_user_amount_type("What type of cashflow is this:") {
                Some(return_val) => {
                    amount = return_val;
                }
                None => {
                    continue;
                }
            };

            break;
        }

        let flow = get_user_flow_type();

        let time_payment = get_user_time_type();

        super::invest_items::cli_disp::CashFlow::new(name.trim(), amount, flow, time_payment)
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

        println!("Present Worth of {}: {}", gradient, result);
    }

    pub fn user_f_from_g_rate() {
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

        let result = match investment_calculations::calculations::f_from_g_rate(
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

    pub fn user_a_from_g_rate() {
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

        let result = match investment_calculations::calculations::a_from_g_rate(
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

        println!("Uniform Payment Equivalent of {}: {}", gradient, result);
    }
}
