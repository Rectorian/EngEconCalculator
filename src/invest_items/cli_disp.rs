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

    pub fn set_color(&mut self, color_val: u8) {
        self.label_color = Some(color_val);
    }
}

//Simple funciton to get the longest label length of all the cashflows
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

//The primary function for printing the legend
pub fn draw_legend(cashflows: &Vec<CashFlow>) {
    let max_len = get_max_label_length(cashflows);

    println!("Maximum label length is: {max_len}");

    let mut legend: Vec<String> = vec![];

    let num_rows: i32 = ((cashflows.len() as i32 - 1) * 2) + 4;

    let num_cols: i32 = max_len as i32 + 3;

    for row in 0..=num_rows {
        let mut col = 0;
        let mut cur_row_string = String::new();

        while col <= num_cols {
            if row == 0 || row == num_rows {
                if col == 0 || col == num_cols {
                    cur_row_string.push('+');
                } else {
                    cur_row_string.push('-');
                }
            } else if (row - 1) % 2 == 0 {
                if col == 0 || col == num_cols {
                    cur_row_string.push('|');
                } else {
                    cur_row_string.push(' ');
                }
            } else {
                if col == 0 || col == num_cols {
                    cur_row_string.push('|');
                } else if col == 2 {
                    //Get the label as a string slice (reference of the label string)
                    let label = match cashflows.get(((row - 2) / 2) as usize) {
                        Some(flow) => flow.label.as_str(),
                        None => " ",
                    };

                    //Adjust the column position before adding creating the colored string. Ensures the
                    //escape codes aren't counted when adjusting the current column position
                    col += label.len() as i32;

                    //Take the current string reference we have and shadow label with the new
                    //colored string
                    let label = match cashflows.get(((row - 2) / 2) as usize) {
                        Some(flow) => match flow.label_color {
                            Some(col) => super::super::ansi_commands::get_text_colored(label, col),
                            None => String::from(label),
                        },
                        None => String::from(label),
                    };

                    //Add the string to the cur_row_string and go to the next loop iteration
                    cur_row_string.push_str(label.as_str());

                    continue;
                } else {
                    cur_row_string.push(' ');
                }
            }

            //Ensure we move to the next column position. The change due to adding the label is
            //handled in that if else statement and uses continue to make sure this doesn't get ran
            col += 1;
        }

        legend.push(cur_row_string);
    }

    for row in legend {
        println!("{}", row);
    }
}
