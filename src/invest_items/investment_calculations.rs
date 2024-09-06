//Much of the functions pieces needed for holding the data needed for our calculations

//The module that holds much of the functions for doing the various calculations
pub mod calculations {
    //Acts like the ../ in file directory for terminal. The first super takes us to the outer scope
    //from the calculations file, which is anything else in the file. The second one takes us to
    //the invest_items module wich has the data module we need as public, although if it wasn't
    //public, it'd still be available
    use super::super::data;

    //A function to calculate the exponential of a f64 type
    fn exponential(base: f64, power: f64) -> f64 {
        if power == 0.0 {
            1.0
        } else if power > 0.0 {
            let mut result = base;

            //println!("Value of result before exponential for loop: {result}");

            for _num in 2i32..=(power as i32) {
                //println!("Value of _num for current iteration: {_num}");
                result *= base;
            }
            result
        } else {
            let mut result = 1.0 / base;
            for _num in (power as i32)..=-2 {
                result /= base;
            }
            result
        }
    }

    //Function to Calcualte Final Amount from initial Principal
    pub fn f_from_p(
        p_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        let p_amount = match p_amount {
            data::AmountType::Principal(amt) => amt.get_f64(),
            _other => {
                return Err(String::from(data::WTA));
            }
        };

        let t_periods = match t_periods {
            data::AmountType::TimePeriods(num_periods) => num_periods.get_f64(),
            _other => {
                return Err(String::from(data::WTP));
            }
        };

        let i_rate = match i_rate {
            data::AmountType::InterestRate(i_type) => i_type,
            _other => {
                return Err(String::from(data::WTI));
            }
        };

        match i_rate {
            data::InterestType::Simple(rate) => Ok(p_amount * (1.0 + (rate * t_periods))),
            data::InterestType::Compound(rate) => Ok(p_amount * exponential(1.0 + rate, t_periods)),
        }
    }

    //Function that calculates the Principal from the final value, interest, and number of periods
    pub fn p_from_f(
        f_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        //The following is quite repetitive due to not being able to make a parameter a specific
        //variant, just of the enum overall type
        let f_amount = match f_amount {
            data::AmountType::Final(amt) => amt.get_f64(),
            _other => {
                return Err(String::from(data::WTA));
            }
        };

        let t_periods = match t_periods {
            data::AmountType::TimePeriods(num_periods) => num_periods.get_f64(),
            _other => {
                return Err(String::from(data::WTP));
            }
        };

        let i_rate = match i_rate {
            data::AmountType::InterestRate(i_type) => i_type,
            _other => {
                return Err(String::from(data::WTI));
            }
        };

        //The main calculation of the function
        match i_rate {
            data::InterestType::Simple(rate) => Ok(f_amount / (1.0 + (rate * t_periods))),
            data::InterestType::Compound(rate) => Ok(f_amount / exponential(1.0 + rate, t_periods)),
        }
    }

    //Function to find P from a uniform series of values A
    pub fn p_from_a(
        a_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        //The following is quite repetitive due to not being able to make a parameter a specific
        //variant, just of the enum overall type
        let a_amount = match a_amount {
            data::AmountType::Uniform(amt) => amt.get_f64(),
            _other => {
                return Err(String::from(data::WTA));
            }
        };

        let t_periods = match t_periods {
            data::AmountType::TimePeriods(num_periods) => num_periods.get_f64(),
            _other => {
                return Err(String::from(data::WTP));
            }
        };

        //Looking back to this part, I think I may be able to piece together what is going on.
        //Because we are passed i_rate by reference, when we match, we unwrap the inner piece to a
        //shadow variable. Because had a reference originally and I didn't annotate the type, we get a reference to the inner
        //object as well to ensure proper lifetimes.
        let i_rate = match i_rate {
            data::AmountType::InterestRate(i_type) => i_type,
            _other => {
                return Err(String::from(data::WTI));
            }
        };

        let i_rate = match i_rate {
            //Why is rate borrowed here causing us to need to de-reference rate?
            //Seems like there is moring going on behind the scenes that I need to understand
            //The first two comparisons don't need it because they just take the value and pull out
            //what is inside them through the funciton I made? They are initially a reference
            //still, but don't require de-referencing to call a method on and the method returns a
            //value
            data::InterestType::Compound(rate) => *rate,
            _other => {
                return Err(String::from(data::WIT));
            }
        };

        //Why is i_rate a reference here?
        //I figured it out. The previous functions I made didn't care about whether or not the
        //numbers were references as they didn't pass them on to another function to do the actual
        //calculation.
        Ok(calc_p_from_a(a_amount, i_rate, t_periods))
    }

    //Actual calculation to help make it simpler
    fn calc_p_from_a(a: f64, i: f64, n: f64) -> f64 {
        let top = exponential(1.0 + i, n) - 1.0;
        let bottom = i * exponential(1.0 + i, n);

        a * (top / bottom)
    }

    pub fn a_from_p(
        p_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        //The following is quite repetitive due to not being able to make a parameter a specific
        //variant, just of the enum overall type
        let p_amount = match p_amount {
            data::AmountType::Principal(amt) => amt.get_f64(),
            _other => {
                return Err(String::from(data::WTA));
            }
        };

        let t_periods = match t_periods {
            data::AmountType::TimePeriods(num_periods) => num_periods.get_f64(),
            _other => {
                return Err(String::from(data::WTP));
            }
        };

        let i_rate = match i_rate {
            data::AmountType::InterestRate(i_type) => i_type,
            _other => {
                return Err(String::from(data::WTI));
            }
        };

        //What a world we live in
        let i_rate = match i_rate {
            data::InterestType::Compound(rate) => *rate,
            _other => {
                return Err(String::from(data::WIT));
            }
        };

        Ok(calc_a_from_p(p_amount, i_rate, t_periods))
    }

    //Actual calculation of A from P
    fn calc_a_from_p(p: f64, i: f64, n: f64) -> f64 {
        let top = i * exponential(1.0 + i, n);
        let bottom = exponential(1.0 + i, n) - 1.0;

        p * (top / bottom)
    }

    //Function to select and run unit_tests
    pub fn unit_test(test_type: &str) -> Result<&str, &str> {
        let test_type = match test_type.to_ascii_lowercase().as_str() {
            "all" => 0,
            "exponential" => 1,
            "uniform payments" => 2,
            _ => -1,
        };

        match test_type {
            -1 => return Err("Invalid Test Type"),
            0 => Ok("All Tested Successfully"),
            1 => {
                unit_test_exp();
                Ok("Successful Test of Exponential Function")
            }
            2 => {
                unit_test_uniform_payments();
                Ok("Successful Test of all Uniform Payment Calculations")
            }
            _ => return Err("Unkown Error"),
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

    fn unit_test_uniform_payments() {
        use super::super::super::ansi_commands::get_text_colored as gtc;
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
                        "Trying f_from_p with\n\tinput 1: {}\n\tinput 2: {}\n\tinput 3:{}\nResult: ",
                        in_1, in_2, in_3
                    );

                    match f_from_p(in_1, in_2, in_3) {
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
