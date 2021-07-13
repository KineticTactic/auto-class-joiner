mod lib;
use crate::lib::*;

fn main() -> ! {
    println!("Starting Auto Class Joiner...");
    run();
    loop {}
    /*hide_console_window();

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
    }*/
}
