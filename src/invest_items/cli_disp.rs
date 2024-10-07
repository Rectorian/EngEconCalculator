const MAX_0_DIST: i8 = 12;

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

impl TimeType {
    pub fn get_range(&self) -> (i32, i32) {
        match self {
            TimeType::Single(num) => (*num, *num),
            TimeType::Multi(vals) => {
                let mut out: (i32, i32) = (0, 0);

                for val in vals {
                    if *val == *vals.get(0).expect("No values in array to find range") {
                        out = (*val, *val);
                    }

                    if *val < out.0 {
                        out.0 = *val;
                    }

                    if *val > out.1 {
                        out.1 = *val;
                    }
                }

                out
            }
            TimeType::Range(range) => *range,
        }
    }
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
        //We need to make sure that when we are getting user input that the time_payment is correct
        //for the amount being input. Ex: If we are adding a uniform amount, then we need a time
        //range, not a single or multi-amount.
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

//The structure that will hold the diagram data
pub struct DiagramData {
    pub time_range: (i32, i32),
    pub relative_max: f64,
    pub flows: Vec<Vec<f64>>,
    pub color: Vec<Vec<u8>>,
}

//Idk what I was going to use this for
enum Increasing {
    Additive(i32),
    Multiplier(f64),
}

impl DiagramData {
    //Function that updates the time range of the DiagramData and moves the vectors accordingly
    pub fn update_time_range(&mut self, new_range: (i32, i32)) {
        //We calculate the number of periods we are adding
        let num_add_periods =
            (new_range.1 - new_range.0 + 1) - (self.time_range.1 - self.time_range.0 + 1);

        //Make sure that we actually need to add periods.
        if num_add_periods <= 0 {
            return;
        }

        //Add the number of empty vectors to the end as needed
        for _ in 0..num_add_periods {
            let empty_vec: Vec<f64> = vec![];
            //Unless push clones the vector, then this will cause memory issues
            self.flows.push(empty_vec);
        }

        //Calculate the number of blank items before the old range's first value
        let num_new_before = self.time_range.0 - new_range.0;
        //Calculate where the last value from the old period is zero indexed
        let old_end = self.time_range.1 - self.time_range.0;

        //Loop through each value in the old range
        for front in 0..=old_end {
            //Start at the end of the data and move into the new blank spaces
            let move_from = old_end - front;

            //Make a copy of the data at old position
            let old_flow = self
                .flows
                .get_mut(move_from as usize)
                .expect("Position of old data out of bounds")
                .clone();

            let old_color = self
                .color
                .get_mut(move_from as usize)
                .expect("Position of old color out of bounds")
                .clone();

            //Re-assign the new position to the data of the old position
            *self
                .flows
                .get_mut((move_from + num_new_before) as usize)
                .expect("Position of where data will be moved out of bounds") = old_flow.clone();

            *self
                .color
                .get_mut((move_from + num_new_before) as usize)
                .expect("Position of where color will be moved out of bounds") = old_color.clone();

            //Clear the old position
            self.flows
                .get_mut(move_from as usize)
                .expect("Position of old data out of bounds")
                .clear();

            self.color
                .get_mut(move_from as usize)
                .expect("Position of old color out of bounds")
                .clear();
        }

        //Ensure the time_range is updated to the new range
        self.time_range = new_range;
    }

    //Funciton to add data from a cashflow to the CFD. Ensures that it handles single, multiple,
    //and range of flows
    pub fn add_flow(&mut self, flow: &CashFlow) {
        //Get the actual flow amount being added
        let mut amt_to_add = flow.amount.get_amount();

        //If the amt being added is larger than the current relative max, then we adjust it
        if amt_to_add > self.relative_max {
            self.relative_max = amt_to_add;
        }

        //If the flow is a payment (negative flow), then it will add it as a negative value
        if let FlowType::Payment = flow.flow {
            amt_to_add *= -1.0;
        }

        //Get the range from the flow
        let amt_range = flow.time_payment.get_range();

        //If the range is larger than the current range for the cfd, then it will expand the range
        if amt_range.0 < self.time_range.0 || amt_range.1 > self.time_range.1 {
            self.update_time_range(amt_range);
        }

        //Add the values according to how they should be added (single, multi, or range)
        match &flow.time_payment {
            TimeType::Single(val) => {
                let add_pos: usize = (val - self.time_range.0) as usize;
                self.flows
                    .get_mut(add_pos)
                    .expect("add_pos for single time out of bounds or vector doesn't exist")
                    .push(amt_to_add);
            }
            TimeType::Range(range) => {
                for time_add in range.0..=range.1 {
                    let add_pos: usize = (time_add - self.time_range.0) as usize;
                    self.flows
                        .get_mut(add_pos)
                        .expect("add_pos for range of time out of bounds or vector doesn't exist")
                        .push(amt_to_add);
                }
            }
            TimeType::Multi(times) => {
                for val in times {
                    let add_pos = (val - self.time_range.0) as usize;
                    self.flows
                        .get_mut(add_pos)
                        .expect("add_pos for multi times values out of bounds or doesn't exist")
                        .push(amt_to_add);
                }
            }
        }
    }
}

//Struct for the actual CFD
pub struct CashFlowDiagram {
    pub name: String,
    pub data: DiagramData,
    pub last_flow_pos: Vec<(i32, i32)>,
    pub flow_spacing: i32,
    pub max_flow_height: i32,
    pub diagram: Vec<String>,
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

pub fn get_max_amount(cashflows: &Vec<CashFlow>) -> f64 {
    use super::data::AmountType;

    let mut max: f64 = 0.0;

    for flow in cashflows {
        let cur_val = match &flow.amount {
            AmountType::Final(amt) => amt.get_f64(),
            AmountType::Principal(amt) => amt.get_f64(),
            AmountType::Uniform(amt) => amt.get_f64(),
            AmountType::Gradient(amt) => amt.get_f64(),
            _ => {
                continue;
            }
        };

        if cur_val > max {
            max = cur_val;
        }
    }

    max
}

pub fn get_max_range(cashflows: &Vec<CashFlow>) -> (i32, i32) {
    if cashflows.len() <= 0 {
        println!("No Flows Input");
        return (0, 0);
    }

    let mut range: (i32, i32);

    match &cashflows
        .get(0)
        .expect("get_max_range cashflows vector is empty after checking its not")
        .time_payment
    {
        TimeType::Single(val) => {
            range = (*val, *val);
        }
        TimeType::Multi(vals) => {
            range = match get_min_max_multi(&vals) {
                Some(vals) => vals,
                None => panic!("No Values Returned"),
            }
        }
        TimeType::Range(vals) => {
            range = *vals;
        }
    }

    for num in 1..(cashflows.len() as i32) {
        match &cashflows
            .get(num as usize)
            .expect("get_max_range cashflows vector is empty after checking its not")
            .time_payment
        {
            TimeType::Single(val) => {
                if *val < range.0 {
                    range.0 = *val;
                } else if *val > range.1 {
                    range.1 = *val;
                }
            }
            TimeType::Multi(vals) => {
                range = match get_min_max_multi(&vals) {
                    Some(vals) => vals,
                    None => panic!("No Values Returned"),
                }
            }
            TimeType::Range(vals) => {
                if vals.0 < range.0 {
                    range.0 = vals.0;
                }

                if vals.1 > range.1 {
                    range.1 = vals.1;
                }
            }
        }
    }

    range
}

fn get_min_max_multi(vals: &Vec<i32>) -> Option<(i32, i32)> {
    if vals.len() == 0 {
        return None;
    }

    let mut max_val = *vals
        .get(0)
        .expect("get_max_multi Vector is empty after checking its not");
    let mut min_val = max_val;

    for val in vals {
        if *val > max_val {
            max_val = *val;
        }
        if *val < min_val {
            min_val = *val;
        }
    }

    Some((min_val, max_val))
}

//The primary function for printing the legend
pub fn draw_legend(cashflows: &Vec<CashFlow>) {
    let max_len = get_max_label_length(cashflows);
    let max_value = get_max_amount(cashflows);
    let max_range = get_max_range(cashflows);

    println!("Maximum label length is: {max_len}");
    println!("Maximum flow value is: {max_value}");
    println!("Maximum time range is: {} to {}", max_range.0, max_range.1);

    let mut legend: Vec<String> = vec![];

    let num_rows: i32 = ((cashflows.len() as i32 - 1) * 2) + 4;

    let num_cols: i32 = max_len as i32 + 3;

    //Treat the Legend like a 2D graph
    //Add the characters corresponding to where in the graph row and col are at
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

pub fn draw_cash_flow_diagram(cashflows: &Vec<CashFlow>) {}
