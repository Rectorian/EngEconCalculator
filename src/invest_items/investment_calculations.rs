//Much of the functions pieces needed for holding the data needed for our calculations

//The module that holds much of the functions for doing the various calculations
pub mod calculations {
    //Acts like the ../ in file directory for terminal. The first super takes us to the outer scope
    //from the calculations file, which is anything else in the file. The second one takes us to
    //the invest_items module wich has the data module we need as public, although if it wasn't
    //public, it'd still be available
    use super::super::data;

    //A function to calculate the exponential of a f64 type
    pub fn exponential(base: f64, power: f64) -> f64 {
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

    //Calculates final value from uniform payments
    pub fn f_from_a(
        a_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
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

        let i_rate = match i_rate {
            data::AmountType::InterestRate(i_type) => i_type,
            _other => {
                return Err(String::from(data::WTI));
            }
        };

        let i_rate = match i_rate {
            data::InterestType::Compound(rate) => *rate,
            _other => {
                return Err(String::from(data::WIT));
            }
        };

        Ok(calc_f_from_a(a_amount, i_rate, t_periods))
    }

    fn calc_f_from_a(a: f64, i: f64, n: f64) -> f64 {
        let top = exponential(1.0 + i, n) - 1.0;
        let bottom = i;

        a * (top / bottom)
    }

    pub fn a_from_f(
        f_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
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

        let i_rate = match i_rate {
            data::InterestType::Compound(rate) => *rate,
            _other => {
                return Err(String::from(data::WIT));
            }
        };

        Ok(calc_a_from_f(f_amount, i_rate, t_periods))
    }

    fn calc_a_from_f(f: f64, i: f64, n: f64) -> f64 {
        let top = i;
        let bottom = exponential(1.0 + i, n) - 1.0;

        f * (top / bottom)
    }

    pub fn p_from_g(
        g_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        let g_amount = match g_amount {
            data::AmountType::Gradient(amt) => amt.get_f64(),
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

        let i_rate = match i_rate {
            data::InterestType::Compound(rate) => *rate,
            _other => {
                return Err(String::from(data::WIT));
            }
        };

        Ok(calc_p_from_g(g_amount, i_rate, t_periods))
    }

    fn calc_p_from_g(g: f64, i: f64, n: f64) -> f64 {
        let top_1 = g;
        let top_2 = exponential(1.0 + i, n) - 1.0;
        let top_3 = n;
        let bot_1 = i;
        let bot_2 = i * exponential(1.0 + i, n);
        let bot_3 = exponential(1.0 + i, n);

        (top_1 / bot_1) * ((top_2 / bot_2) - (top_3 / bot_3))
    }

    pub fn a_from_g(
        g_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        let g_amount = match g_amount {
            data::AmountType::Gradient(amt) => amt.get_f64(),
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

        let i_rate = match i_rate {
            data::InterestType::Compound(rate) => *rate,
            _other => {
                return Err(String::from(data::WIT));
            }
        };

        Ok(calc_a_from_g(g_amount, i_rate, t_periods))
    }

    fn calc_a_from_g(g: f64, i: f64, n: f64) -> f64 {
        let top_1 = 1.0;
        let top_2 = n;
        let bot_1 = i;
        let bot_2 = exponential(1.0 + i, n) - 1.0;

        g * ((top_1 / bot_1) - (top_2 / bot_2))
    }

    pub fn f_from_g(
        g_amount: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        let g_amount = match g_amount {
            data::AmountType::Gradient(amt) => amt.get_f64(),
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

        let i_rate = match i_rate {
            data::InterestType::Compound(rate) => *rate,
            _other => {
                return Err(String::from(data::WIT));
            }
        };

        Ok(calc_f_from_g(g_amount, i_rate, t_periods))
    }

    fn calc_f_from_g(g: f64, i: f64, n: f64) -> f64 {
        let term_1 = 1.0 / i;
        let term_2 = (exponential(1.0 + i, n) - 1.0) / i;
        let term_3 = (1.0 / i) * n;

        g * ((term_1 * term_2) - term_3)
    }

    pub fn p_from_g_rate(
        a_initial: &data::AmountType,
        g_rate: &data::AmountType,
        t_periods: &data::AmountType,
        i_rate: &data::AmountType,
    ) -> Result<f64, String> {
        let a_initial = match a_initial {
            data::AmountType::Uniform(amt) => amt.get_f64(),
            _other => {
                return Err(String::from(data::WTA));
            }
        };

        let g_rate = match g_rate {
            data::AmountType::GradientRate(rate_type) => rate_type,
            _other => {
                return Err(String::from(data::WTI));
            }
        };

        let g_rate = match g_rate {
            data::InterestType::Compound(amt) => *amt,
            _other => {
                return Err(String::from(data::WIT));
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

        let i_rate = match i_rate {
            data::InterestType::Compound(rate) => *rate,
            _other => {
                return Err(String::from(data::WIT));
            }
        };

        Ok(calc_p_from_g_rate(a_initial, g_rate, i_rate, t_periods))
    }

    fn calc_p_from_g_rate(a: f64, g: f64, i: f64, n: f64) -> f64 {
        if g == i {
            (n * a) / (1.0 + i)
        } else {
            let term_1 = exponential((1.0 + g) / (1.0 + i), n);
            let term_2 = 1.0 - term_1;
            let term_3 = term_2 / (i - g);

            a * term_3
        }
    }
}

pub mod unit_tests;
