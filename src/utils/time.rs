const MINUTE: usize = 60;
const HOUR: usize = MINUTE * 60;
const DAY: usize = HOUR * 24;
const YEAR: usize = DAY * 365;

pub fn format_time_ord(seconds: usize) -> String {
    let year = seconds / YEAR;
    let day = seconds % YEAR / DAY;
    let hour = seconds % DAY / HOUR;

    if year > 0 {
        format!(
            "{year}{} year and {day}{} day",
            ord_indicator(year),
            ord_indicator(day)
        )
    } else {
        format!(
            "{day}{} day and {hour}{} hour",
            ord_indicator(day),
            ord_indicator(hour)
        )
    }
}

pub fn format_time(seconds: usize) -> String {
    let years = seconds / YEAR;
    let days = seconds % YEAR / DAY;
    let hours = seconds % DAY / HOUR;
    let minutes = seconds % HOUR / MINUTE;
    let seconds = seconds % MINUTE;

    let year_s = if years == 1 { "" } else { "s" };
    let day_s = if days == 1 { "" } else { "s" };
    let hour_s = if hours == 1 { "" } else { "s" };
    let minute_s = if minutes == 1 { "" } else { "s" };
    let second_s = if seconds == 1 { "" } else { "s" };

    if years > 0 && days > 0 {
        format!("{years} year{year_s} and {days} day{day_s}")
    } else if years > 0 {
        format!("{years} year{year_s}")
    } else if days > 0 && hours > 0 {
        format!("{days} day{day_s} and {hours} hour{hour_s}")
    } else if days > 0 {
        format!("{days} day{day_s}")
    } else if hours > 0 && minutes > 0 {
        format!("{hours} hour{hour_s} and {minutes} minute{minute_s}")
    } else if hours > 0 {
        format!("{hours} hour{hour_s}")
    } else if minutes > 0 && seconds > 0 {
        format!("{minutes} minute{minute_s} and {seconds} second{second_s}")
    } else if minutes > 0 {
        format!("{minutes} minute{minute_s}")
    } else {
        format!("{seconds} second{second_s}")
    }
}

fn ord_indicator(num: usize) -> &'static str {
    if let 11..=13 = num % 100 {
        return "th";
    }

    match num % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}
