use chrono::prelude::*;

#[derive(Debug)]
struct SmallStreamedRecord {
    pub show_name: String,
    pub view_date: NaiveDate,
    pub category: String,
    pub charge_amount: isize,
}

fn main() {
    let record = SmallStreamedRecord {
        show_name: "Motown Magic".to_string(),
        view_date: NaiveDate::from_ymd_opt(2023, 2, 5).unwrap(),
        category: "Kids".to_string(),
        charge_amount: 1489,
    };
    println!("record: {:?}", record);
}
