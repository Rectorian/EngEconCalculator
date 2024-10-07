//Enum type for storing different number types
pub enum Amount {
    Fl32(f32),
    Fl64(f64),
    In32(i32),
    In64(i64),
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amount::Fl32(num) => write!(f, "{}", num),
            Amount::Fl64(num) => write!(f, "{}", num),
            Amount::In32(num) => write!(f, "{}", num),
            Amount::In64(num) => write!(f, "{}", num),
        }
    }
}

impl Clone for Amount {
    fn clone(&self) -> Amount {
        match *self {
            Amount::Fl32(num) => Amount::Fl32(num),
            Amount::Fl64(num) => Amount::Fl64(num),
            Amount::In32(num) => Amount::In32(num),
            Amount::In64(num) => Amount::In64(num),
        }
    }
}

//A function for the Amount enum to make sure we can get the same type of number for accurate
//calculations
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

//Enum that holds the interest type, used for doing proper calculations
pub enum InterestType {
    Simple(f64),
    Compound(f64),
}

//Implementing a function to get the value out of the InterestType enum quickly
impl InterestType {
    pub fn get_f64(&self) -> f64 {
        match self {
            InterestType::Simple(num) => *num,
            InterestType::Compound(num) => *num,
        }
    }
}

impl std::fmt::Display for InterestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterestType::Simple(num) => write!(f, "%{} Simple", num),
            InterestType::Compound(num) => write!(f, "%{} Compound", num),
        }
    }
}

impl Clone for InterestType {
    fn clone(&self) -> InterestType {
        match *self {
            InterestType::Simple(num) => InterestType::Simple(num),
            InterestType::Compound(num) => InterestType::Compound(num),
        }
    }
}

//Enum for holding the different types of amounts
pub enum AmountType {
    Principal(Amount),
    Final(Amount),
    Uniform(Amount),
    Gradient(Amount),
    GradientRate(super::data::InterestType),
    TimePeriods(Amount),
    InterestRate(super::data::InterestType),
}

//Implementing a function to quickly get the value out of an AmountType as an f64
impl AmountType {
    pub fn get_amount(&self) -> f64 {
        match self {
            AmountType::GradientRate(inter) | AmountType::InterestRate(inter) => inter.get_f64(),
            AmountType::Final(amt) => amt.get_f64(),
            AmountType::Principal(amt) => amt.get_f64(),
            AmountType::Uniform(amt) => amt.get_f64(),
            AmountType::Gradient(amt) => amt.get_f64(),
            AmountType::TimePeriods(amt) => amt.get_f64(),
        }
    }
}
impl std::fmt::Display for AmountType {
    //We may be able to re-write by creating a string that we have our prepend on and then passing
    //that to the formatter of the bundled data as it seems write just adds onto the buffer given?
    //(need to confirm)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Get what we want to prepend before printing our values
        let prepend;
        match self {
            AmountType::Principal(_amt) => prepend = "Principal: ",
            AmountType::Final(_amt) => prepend = "Final Amount: ",
            AmountType::Uniform(_amt) => prepend = "Uniform Payment: ",
            AmountType::Gradient(_amt) => prepend = "Gradient Payment: ",
            AmountType::GradientRate(_rate) => prepend = "Gradient Rate: ",
            AmountType::TimePeriods(_periods) => prepend = "Number of Periods: ",
            AmountType::InterestRate(_rate) => prepend = "Interest Rate: ",
        };

        //Write the prepend to the formatter, handle error if necessary
        if let Err(err_mes) = write!(f, "{}", prepend) {
            return Err(err_mes);
        }

        //Now we can use display of the bundled data.
        match self {
            AmountType::Principal(amt) => amt.fmt(f),
            AmountType::Final(amt) => amt.fmt(f),
            AmountType::Uniform(amt) => amt.fmt(f),
            AmountType::Gradient(amt) => amt.fmt(f),
            AmountType::GradientRate(rate) => rate.fmt(f),
            AmountType::TimePeriods(periods) => periods.fmt(f),
            AmountType::InterestRate(rate) => rate.fmt(f),
        }
    }
}

impl Clone for AmountType {
    fn clone(&self) -> AmountType {
        //I get an error I don't fully understand yet when I de-reference self
        //From what I understand is that when you dereference in the match, it tries to create
        //a copy of the data held within when you capture the data. If that data isn't a
        //primitive type, it can't do so and will then capture it as a reference?
        match self {
            AmountType::Principal(amt) => AmountType::Principal(amt.clone()),
            AmountType::Final(amt) => AmountType::Final(amt.clone()),
            AmountType::Uniform(amt) => AmountType::Uniform(amt.clone()),
            AmountType::Gradient(amt) => AmountType::Gradient(amt.clone()),
            AmountType::GradientRate(rate) => AmountType::GradientRate(rate.clone()),
            AmountType::TimePeriods(periods) => AmountType::TimePeriods(periods.clone()),
            AmountType::InterestRate(rate) => AmountType::InterestRate(rate.clone()),
        }
    }
}

pub const WTA: &str = "Invalid Type for Amount";
pub const WTP: &str = "Invalid Type for Time Periods";
pub const WTI: &str = "Invalid Type for Interest Rate";
pub const WIT: &str = "Invalid Interest Rate Type (Likely needs to be Compound)";
