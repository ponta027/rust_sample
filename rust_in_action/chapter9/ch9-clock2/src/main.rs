#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
#[cfg(windows)]
use winapi;

//
use chrono::{DateTime, Local, TimeZone};
use clap::{App, Arg};
use std::mem::zeroed;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }
    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {}

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{settimeofday, timezone};
        use libc::{suseconds_t, time_t, timeval};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
        //
    }
}

fn main() {
    println!("Hello, world!");
    let app = App::new("clock")
        .version("0.1.2")
        .about("")
        .after_help("")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .takes_value(true)
                .long("use-standard")
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(
            Arg::with_name("datetime")
                .help("When <action> is 'set' , apply <datetime>.   Otherwise , ignore"),
        );

    let args = app.get_matches();

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();
        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse {} according to {} ", t_, std);
        let t = parser(t_).expect(&err_msg);
        Clock::set(t);
        let maybe_error = std::io::Error::last_os_error();
        let os_error_code = &maybe_error.raw_os_error();
        match os_error_code {
            Some(0) => (),
            Some(_) => eprintln!("Unable to set the time : {:?}", maybe_error),
            None => (),
        }
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unimplemented!(),
    }
}
