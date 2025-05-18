use chrono::prelude::*;

fn main() {
    let utc = Utc::now();
    let local = Local::now();
    println!("The Current time is now {} ",local.format("The Year is %Y and the Month is %m and the Day is %d")); 
    println!("This is for UTC {}",utc.format("The Year is %Y"));
    println!("Let's explore the custom dates and timing now!");
    let date = NaiveDate::from_ymd_opt(2005, 10, 28).unwrap();
    println!("The Date of Hello world is {} ",date);
    let date_time = date.and_hms_opt(10, 10, 10).unwrap();
    println!("The Time is {}" , date_time);
}