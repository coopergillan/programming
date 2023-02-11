use chrono::NaiveDate;
use fixed_width::{from_bytes, LineBreak, Reader};
use fixed_width_derive::FixedWidth;
use serde::Serialize;
use serde_derive::Deserialize;
use std::error::Error;
use std::io;
use std::process;
use std::result;

#[derive(Debug, FixedWidth, Deserialize, Serialize)]
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

fn run() -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_reader(io::stdin())
        .width(44)
        .linebreak(LineBreak::Newline);
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(io::stdout());

    for record in reader.byte_reader().filter_map(result::Result::ok) {
        let streamed_record: SmallStreamedRecord = from_bytes(&record)?;
        wtr.serialize(streamed_record)?;
        wtr.flush()?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("Got error: {}", err);
        process::exit(1);
    }
}
