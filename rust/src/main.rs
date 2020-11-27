mod is_bree;
use argopt::cmd;
use datetime::{LocalDate, Month, ISO};
use is_bree::is_bree;
use std::error::Error;

#[cmd]

fn main(year: i64, month: i8, day: i8) -> Result<(), Box<dyn Error>> {
    let date = LocalDate::ymd(year, Month::from_one(month)?, day)?;
    let bree = match is_bree(date) {
        true => "bree",
        false => "not bree",
    };
    println!("{} is {}", date.iso(), bree);
    Ok(())
}
