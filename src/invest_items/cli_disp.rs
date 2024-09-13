use super::data;

pub enum FlowType {
    Payment,
    Withdrawal,
}

pub enum TimeType {
    Single(i32),
    Multi(Vec<i32>),
    Range((i32, i32)),
}

pub struct CashFlow {
    pub label: String,
    pub amount: data::AmountType,
    pub flow: FlowType,
    pub time_payment: TimeType,
    pub label_color: Option<u8>,
}

impl CashFlow {
    //We will create a string object via a &str and the rest we will take ownership of the items
    //passed into the function
    pub fn new(
        flow_name: &str,
        amount: data::AmountType,
        flow: FlowType,
        time_payment: TimeType,
    ) -> Self {
        CashFlow {
            label: String::from(flow_name),
            amount,
            flow,
            time_payment,
            label_color: None,
        }
    }
}

pub fn get_max_label_length(cashflows: &Vec<CashFlow>) -> u32 {
    let mut max: u32 = 0;

    for flow in cashflows {
        let cur_len = flow.label.len() as u32;

        if cur_len > max {
            max = cur_len;
        }
    }

    max
}

pub fn draw_legend(cashflows: &Vec<CashFlow>) {
    let max_len = get_max_label_length(cashflows);

    println!("Maximum label length is: {max_len}");
}
