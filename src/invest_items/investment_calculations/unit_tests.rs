use super::calculations::*;

//Function to select and run unit_tests
pub fn unit_test(test_type: &str) -> Result<&str, &str> {
    let test_type = match test_type.to_ascii_lowercase().as_str() {
        "all" => 0,
        "exponential" => 1,
        "uniform payments" => 2,
        _ => -1,
    };

    match test_type {
        -1 => Err("Invalid Test Type"),
        0 => Ok("All Tested Successfully"),
        1 => {
            unit_test_exp();
            Ok("Successful Test of Exponential Function")
        }
        2 => {
            unit_test_uniform_payments(false);
            Ok("All Uniform Payment Caclulations Have Been Ran: Please Check For Correctness")
        }
        _ => Err("Unkown Error"),
    }
}

fn unit_test_exp() {
    let t_1 = exponential(5.0, 3.0);
    assert!(t_1 == 125.0, "\t5^3 did not equal 125, Was {t_1}");

    let t_2 = exponential(5.0, 1.0);
    assert!(t_2 == 5.0, "\t5^1 did not equal 5, Was {t_2}");

    let t_3 = exponential(5.0, 0.0);
    assert!(t_3 == 1.0, "\t5^0 did not equal 1, Was {t_3}");

    let t_4 = exponential(5.0, -1.0);
    assert!(
        t_4 == (1.0 / 5.0) as f64,
        "\t5^-1 did not equal 0.2, Was {t_4}"
    );
}

fn unit_test_uniform_payments(limited: bool) {
    if !limited {
        use super::super::super::ansi_commands::get_text_colored as gtc;
        use super::super::data;
        let inputs: Vec<data::AmountType> = vec![
            data::AmountType::Final(data::Amount::Fl64(100000.0)),
            data::AmountType::Principal(data::Amount::Fl64(50000.0)),
            data::AmountType::Uniform(data::Amount::In64(5000)),
            data::AmountType::TimePeriods(data::Amount::In64(5)),
            data::AmountType::InterestRate(data::InterestType::Simple(0.20)),
            data::AmountType::InterestRate(data::InterestType::Compound(0.12)),
        ];

        let inputs_2 = inputs.clone();
        let inputs_3 = inputs.clone();

        for in_1 in &inputs {
            for in_2 in &inputs_2 {
                for in_3 in &inputs_3 {
                    print!(
                        "Trying with inputs:\n\tinput 1: {}\n\tinput 2: {}\n\tinput 3: {}\nResult:\n",
                        in_1, in_2, in_3
                    );

                    print!("\tf_from_p: ");
                    match f_from_p(in_1, in_2, in_3) {
                        Ok(num) => {
                            let temp = format!("${:.2}", num);
                            println!("{}", gtc(temp.as_str(), 10));
                        }
                        Err(error_message) => {
                            println!("{}", gtc(error_message.as_str(), 9));
                        }
                    }

                    print!("\tp_from_f: ");
                    match p_from_f(in_1, in_2, in_3) {
                        Ok(num) => {
                            let temp = format!("${:.2}", num);
                            println!("{}", gtc(temp.as_str(), 10));
                        }
                        Err(error_message) => {
                            println!("{}", gtc(error_message.as_str(), 9));
                        }
                    }

                    print!("\tp_from_a: ");
                    match p_from_a(in_1, in_2, in_3) {
                        Ok(num) => {
                            let temp = format!("${:.2}", num);
                            println!("{}", gtc(temp.as_str(), 10));
                        }
                        Err(error_message) => {
                            println!("{}", gtc(error_message.as_str(), 9));
                        }
                    }

                    print!("\ta_from_p: ");
                    match a_from_p(in_1, in_2, in_3) {
                        Ok(num) => {
                            let temp = format!("${:.2}", num);
                            println!("{}", gtc(temp.as_str(), 10));
                        }
                        Err(error_message) => {
                            println!("{}", gtc(error_message.as_str(), 9));
                        }
                    }

                    print!("\tf_from_a: ");
                    match f_from_a(in_1, in_2, in_3) {
                        Ok(num) => {
                            let temp = format!("${:.2}", num);
                            println!("{}", gtc(temp.as_str(), 10));
                        }
                        Err(error_message) => {
                            println!("{}", gtc(error_message.as_str(), 9));
                        }
                    }

                    print!("\ta_from_f: ");
                    match a_from_f(in_1, in_2, in_3) {
                        Ok(num) => {
                            let temp = format!("${:.2}", num);
                            println!("{}", gtc(temp.as_str(), 10));
                        }
                        Err(error_message) => {
                            println!("{}", gtc(error_message.as_str(), 9));
                        }
                    }
                }
            }
        }
    }
}
