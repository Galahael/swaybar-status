use chrono::{Datelike, Local, Timelike, Weekday};

pub fn present_time() -> String {
    let now = Local::now();

    let weekday = match now.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturaday",
        Weekday::Sun => "Sunday",
    };

    let month = match now.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "Aug",
        9 => "Sept",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => unreachable!(),
    };

    let day = now.day();
    let hour = now.hour();

    // Formats a '0' in front of single-digit minutes:
    let minute = match now.minute() {
        x if x < 10 => format!("0{}", x),
        x if x >= 10 => format!("{}", x),
        _ => String::from("now.minute() error"),
    };

    format!("{}, {} {} | {}:{}", weekday, month, day, hour, minute)
}
