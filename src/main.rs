extern crate cronjob;

use chrono::Datelike;
use cronjob::CronJob;
use notify_rust::Notification;
use std::{fs, ops::Add};
use webbrowser;

fn get_json(filename: &str) -> serde_json::Value {
    let raw_string = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => {
            send_notif(&String::from("Error reading JSON file: ").add(filename).add("\n Make sure that the specified file exists.").add(&e.to_string()));
            std::process::exit(1);
        }
    };
    match serde_json::from_str(&raw_string) {
        Ok(v) => v,
        Err(e) => {
            send_notif(&String::from("Error while parsing JSON file: ").add(filename).add("\nMake sure that the formatting is correct.\n").add(&e.to_string()));
            std::process::exit(1);
        }
    }
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

#[cfg(feature = "hide_console")]
fn hide_console_window() {
    unsafe { winapi::um::wincon::FreeConsole() };
}

#[cfg(not(feature = "hide_console"))]
fn hide_console_window() {}

fn send_notif(msg: &str ) {
    Notification::new()
        .appname("AutoClassJoiner-Rust")
        .summary(msg)
        .show().expect("Cannot send notification!!!");
}

fn main() {
    println!("Starting AutoClassJoiner");

    hide_console_window();

    send_notif("AutoClassJoiner-Rust has started!");

    let timings_json = get_json("data/timings.json");
    let timings_raw = timings_json["timings"].as_array().unwrap();

    let mut timings: Vec<(u32, u32)> = Vec::new();

    for e in timings_raw.iter() {
        let e_to_str =e.to_string();
        let time = rem_first_and_last(&e_to_str);
        let mut splitted = time.split(":");
        let tuple:(u32, u32) = (splitted.next().unwrap().parse::<u32>().unwrap(), splitted.next().unwrap().parse::<u32>().unwrap());
        timings.push(tuple);
    }
    
    let timezone_str = &chrono::Local::now().offset().to_string()[1..];
    let mut timezone_str_split = timezone_str.split(":");
    let timezone = timezone_str_split.next().unwrap().parse::<i32>().unwrap() * 3600 + timezone_str_split.next().unwrap().parse::<i32>().unwrap() * 60;
    let timezone = match &timezone_str[..1] {
        "-" => -timezone,
        _ => timezone
    };
    
    for (i, (hour, minute)) in timings.iter().enumerate() {
        let mut job = CronJob::new(&String::from("Class Index ").add(&i.to_string()),  move |_: &str| {
            let classes = get_json("data/classes.json");
            let links = get_json("data/links.json");

            let weekday = chrono::offset::Local::now().weekday().to_string();

            let current_class = classes[weekday][i].to_string();
            let current_class = rem_first_and_last(&current_class);

            if current_class == "" {
                return;
            }
            
            let class_link = &links[&current_class].to_string();
            let class_link = rem_first_and_last(&class_link);
            
            if webbrowser::open(&class_link).is_ok() {
                send_notif(&String::from("Joining Class ").add(&current_class));
            }
        });
        job.minutes(&minute.to_string());
        job.hours(&hour.to_string());
        job.seconds("0");
        job.offset(timezone);
    
        CronJob::start_job_threaded(job);
    }

    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

}