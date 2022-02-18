use ansi_term::Colour::Red;
use chrono::{DateTime, Duration, FixedOffset, Local, Utc};
use chrono::{Datelike, Timelike};
use std::thread;
use std::time::Instant;

fn main() {
    let breakline = Red
        .paint("------------------------------------------------------------")
        .to_string();

    println!("{}", breakline);
    elapsed_time();

    println!("{}", breakline);
    perform_checked_date_and_time_calculations();

    println!("{}", breakline);
    convert_local_time_to_another_timezone();

    println!("{}", breakline);
    examine_date_and_time();
    println!("{}", breakline);
}

fn expensive_function() {
    thread::sleep(std::time::Duration::from_secs(1));
}

/// # 测量两个代码段块之间的使用时间
/// 使用标准`time::Instant::elapsed`获取`time::Instant::now`开始的时间间隔
fn elapsed_time() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("调用耗时方法使用的时间：{:?}", duration);
}

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

fn perform_checked_date_and_time_calculations() {
    let now = Utc::now();
    println!("Utc::now: {}", now);

    let almost_three_weeks_from_now = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}

fn convert_local_time_to_another_timezone() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time.naive_utc());
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!(
        "Time in Rio De Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone)
    );
}

fn examine_date_and_time() {
    let now = Utc::now();

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02}:{}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
}
