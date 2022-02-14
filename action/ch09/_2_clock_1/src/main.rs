#![allow(unused)]

use chrono::{DateTime, Local};
use clap::{App, Arg};

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

fn main() {
    let app = App::new("Clock").version("1.0").about("Gets and sets (aspirationally) the time.")
        .arg(
            Arg::new("action")
            .takes_value(true)
            .possible_values(&["get", "set"])
            .default_value("get"))
        .arg(
            Arg::new("std").short('s')
            .long("use-standard")
            .takes_value(true)
            .possible_values(&["rfc2822", "rfc3339", "timestamp"])
            .default_value("rfc3339"))
        .arg(
            Arg::new("datetime")
            .help("When <action> is 'set', apply <datetime>. Otherwise, ignore.")
    );

    let args = app.get_matches();

    let action = args.value_of("std").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        unimplemented!() // break early
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unimplemented!(),
    };
}
