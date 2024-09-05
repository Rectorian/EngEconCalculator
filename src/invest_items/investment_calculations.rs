pub mod data {
    pub enum Amount {
        Fl32(f32),
        Fl64(f64),
        In32(i32),
        In64(i64),
    }

    impl Amount {
        pub fn get_f64(&self) -> f64 {
            //Interestingly, we need to dereference the enum since we borrowed it
            match *self {
                Amount::Fl32(num) => num as f64,
                Amount::Fl64(num) => num,
                Amount::In32(num) => num as f64,
                Amount::In64(num) => num as f64,
            }
        }
    }

    pub enum InterestType {
        Simple(f64),
        Compound(f64),
    }

    pub enum AmountType {
        Principal(Amount),
        Final(Amount),
        Uniform(Amount),
        Gradient(Amount),
        GradientRate(super::data::InterestType),
        TimePeriods(Amount),
        InterestRate(super::data::InterestType),
    }
}

pub mod calculations {
    use super::data;

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

    //Function that calculates the Principal from the final value, interest, and number of periods
    pub fn p_from_f(
        f_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        let f_amount = match f_amount {
            data::AmountType::Final(amt) => amt.get_f64(),
            _other => {
                return Err(String::from("Invalid Amount Type"));
            }
        };

        let t_periods = match t_periods {
            data::AmountType::TimePeriods(num_periods) => num_periods.get_f64(),
            _other => {
                return Err(String::from("Invalid Type for Time Periods"));
            }
        };

        let i_rate = match i_rate {
            data::AmountType::InterestRate(i_type) => i_type,
            _other => {
                return Err(String::from("Invalid type for Interest Rate"));
            }
        };

        match i_rate {
            data::InterestType::Simple(rate) => Ok(f_amount / (1.0 + (rate * t_periods))),
            data::InterestType::Compound(rate) => Ok(f_amount / exponential(1.0 + rate, t_periods)),
        }
    }

    pub fn unit_test(test_type: &str) -> Result<&str, &str> {
        let test_type = match test_type.to_ascii_lowercase().as_str() {
            "all" => 0,
            "exponential" => 1,
            _ => -1,
        };

        match test_type {
            -1 => return Err("Invalid Test Type"),
            0 => Ok("All Tested Successfully"),
            1 => {
                unit_test_exp();
                Ok("Successful Test of Exponential Function")
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

        let t_4 = exponential(5.0, -2.0);
        assert!(
            t_4 == (1.0 / 5.0) as f64,
            "\t5^-1 did not equal 0.2, Was {t_4}"
        );
    }
}
