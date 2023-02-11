use chrono::NaiveDate;
use fixed_width_derive::FixedWidth;
// use serde::Serialize;
use serde_derive::Deserialize;
use std::error::Error;
// use std::io;
use std::process;

#[allow(dead_code)]
#[allow(unused_attributes)]
#[derive(Debug, FixedWidth, Deserialize)]
struct SmallStreamedRecord {
    #[fixed_width(range = "0..20")]
    pub show_name: String,
    #[fixed_width(range = "20..30")]
    pub view_date: NaiveDate,
    #[fixed_width(range = "30..40")]
    pub category: String,
    #[fixed_width(range = "40..44")]
    pub charge_amount: isize,
}

// #[derive(Debug, Deserialize)]
// struct DateRecord {
//     pub view_date: Option<NaiveDate>,
// }
//
// impl OrigFixedWidth for DateRecord {
//     fn fields() -> FieldSet {
//         FieldSet::Seq(vec![FieldSet::new_field(20..30)])
//     }
// }

fn run() -> Result<(), Box<dyn Error>> {
    let raw_data = "        MOTOWN MAGIC2023-02-05      KIDS1489";
    let raw_width = raw_data.len();

    let mut reader = fixed_width::Reader::from_string(raw_data).width(raw_width);

    for record in reader.byte_reader().filter_map(std::result::Result::ok) {
        let streamed_record: SmallStreamedRecord = fixed_width::from_bytes(&record)?;
        println!("streamed_record: {:?}", streamed_record);
    }

    // let record = SmallStreamedRecord {
    //     show_name: "MOTOWN MAGIC".to_string(),
    //     view_date: NaiveDate::from_ymd_opt(2023, 2, 5).unwrap(),
    //     category: "KIDS".to_string(),
    //     charge_amount: 1489,
    // };
    // println!("record: {:?}", record);
    //
    // let mut wtr = csv::WriterBuilder::new()
    //     // .quote_style(csv::QuoteStyle::NonNumeric)
    //     .from_writer(io::stdout());
    //
    // wtr.serialize(record)?;
    // wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
