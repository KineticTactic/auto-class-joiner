use chrono::Datelike;
use cronjob::CronJob;
use notify_rust::Notification;
use std::fs;
use std::process;
use webbrowser;

pub fn run() {
    let classes = Class::new();

    for class in classes {
        println!("Loading class {} at {}:{} with url {}", class.name, class.hour, class.minute, class.link);

        let minute = &class.minute.clone();
        let hour = &class.hour.clone();

        let mut job = CronJob::new(
            &format!("Class Name: {}", &class.name.clone()),
            move |_: &str| {
                class.open_class().unwrap_or_else(|err| {
                    send_notif(&format!("An error occured: {}", err));
                    process::exit(1)
                })
            },
        );

        job.minutes(minute);
        job.hours(hour);
        job.seconds("0");
        job.offset(get_timezone());

        CronJob::start_job_threaded(job);
    }

    send_notif("auto-class-joiner has started successfully!");

    println!("Freeing Console Window...");
    hide_windows_console_window();

}

pub struct Class {
    hour: String,
    minute: String,
    link: String,
    name: String,
}

impl Class {
    pub fn new() -> Vec<Class> {
        let timings_json = get_json("data/timings.json");
        let timings_set = timings_json["timings"].as_array().unwrap().iter().map(|s| {
            (
                s.to_string()
                    .split(':')
                    .nth(0)
                    .unwrap_or_else(|| {
                        send_notif("Invalid minute entry in timings file");
                        process::exit(1)
                    })
                    .to_string(),
                s.to_string()
                    .split(':')
                    .nth(1)
                    .unwrap_or_else(|| {
                        send_notif("Invalid hour entry in timings file");
                        process::exit(1)
                    })
                    .to_string(),
            )
        });

        let weekday = chrono::offset::Local::now().weekday().to_string();

        let links_json = get_json("data/links.json");

        let class_json = get_json("data/classes.json");
        let class_set = class_json[weekday]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.to_string());

        timings_set
            .zip(class_set)
            .map(|((hour, minute), name): ((String, String), String)| Class {
                minute: minute.trim_matches('"').to_owned(),

                hour: hour.trim_matches('"').to_owned(),

                // tfw javascript moment
                link: match links_json[name.trim_matches('"')]
                    .to_string()
                    .trim_matches('"')
                    .to_owned()
                {
                    x if x == "null" => {
                        send_notif(&format!("No link for {} in links file", name));
                        process::exit(1)
                    }

                    x => x,
                },

                name: name.trim_matches('"').to_owned(),
            })
            .collect()
    }

    pub fn open_class(&self) -> Result<(), String> {
        send_notif(&format!("Joining Class: {}", self.name));

        match webbrowser::open(&self.link) {
            Ok(_) => Ok(()),
            Err(x) => Err(x.to_string()),
        }
    }
}

fn get_json(filename: &str) -> serde_json::Value {
    let raw_string = fs::read_to_string(filename).unwrap_or_else(|_| {
        send_notif(&format!(
            "Error reading JSON file: {}\nMake sure that the specified file exists.",
            filename
        ));
        std::process::exit(1);
    });
    serde_json::from_str(&raw_string).unwrap_or_else(|_| {
        send_notif(&format!(
            "Error while parsing JSON file: {}\nMake sure that the formatting is correct.",
            filename
        ));
        std::process::exit(1);
    })
}

#[cfg(feature = "hide_console")]
fn hide_windows_console_window() {
    unsafe { winapi::um::wincon::FreeConsole() };
}

#[cfg(not(feature = "hide_console"))]
fn hide_windows_console_window() {}

fn send_notif(msg: &str) {
    Notification::new()
        .appname("Auto Class Joiner")
        .summary(msg)
        .show()
        .expect("Cannot send notification");
}

fn get_timezone() -> i32 {
    let timezone_str = &chrono::Local::now().offset().to_string()[1..];

    let mut timezone_str_split = timezone_str.split(":");

    let timezone = timezone_str_split.next().unwrap().parse::<i32>().unwrap() * 3600
        + timezone_str_split.next().unwrap().parse::<i32>().unwrap() * 60;

    match &timezone_str[..1] {
        "-" => -timezone,
        _ => timezone,
    }
}
