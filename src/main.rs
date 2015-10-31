#![feature(convert)]

use std::collections::HashMap;
use std::fmt;

struct Task {
    time: u64,
    task: String,
}

impl Task {
    fn new(time: u64, task: String) -> Task {
        Task {
            time: time,
            task: task,
        }
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", iso_to_time(self.time), self.task)
    }
}

struct Calendar {
    tasks: HashMap<u64, Vec<Task>>,
}

impl Calendar {
    fn new() -> Calendar {
        Calendar {
            tasks: HashMap::new(),
        }
    }

    fn add_task(&mut self, date: u64, task: Task) {
        if self.tasks.get(&date).is_none() {
            let mut tmp_vec = Vec::new();
            tmp_vec.push(task);
            self.tasks.insert(date, tmp_vec);
        } else {
            self.tasks.get_mut(&date).unwrap().push(task);
        }
    }

    fn check_tasks(&self, date: u64) -> &Vec<Task> {
        self.tasks.get(&date).unwrap()
    }
}

fn date_fmt(date_str: &str) -> u64 {
    let tokens: Vec<&str> = date_str.split_whitespace().collect();
    let mut month = 0;

    match tokens[0].to_lowercase().as_str() {
        "january" => { month = 1; },
        "febuary" => { month = 2; },
        "march" => { month = 3; },
        "april" => { month = 4; },
        "may" => { month = 5; },
        "june" => { month = 6; },
        "july" => { month = 7; },
        "august" => { month = 8; },
        "september" => { month = 9; },
        "october" => { month = 10; },
        "november" => { month = 11; },
        "december" => { month = 12; },
        _ => {},
    }

    let day_str: String = tokens[1].matches(char::is_numeric).collect();
    let day = day_str.parse::<u8>().unwrap();

    let year_str: String = tokens[2].matches(char::is_numeric).collect();
    let year = year_str.parse::<u32>().unwrap();

    let date = format!("{:02}{:02}{:04}", month, day, year);
    date.parse::<u64>().unwrap()
}

// hh:mm:ss
fn iso_to_time(iso_time: u64) -> String {
    let tmp_string = iso_time.to_string();

    let hours_str = tmp_string.split_at(2).0;
    let min_str = tmp_string.split_at(2).1.split_at(2).0;
    let sec_str = tmp_string.split_at(2).1.split_at(2).1;

    let time = format!("{}:{}:{}", hours_str, min_str, sec_str);
    time
}

fn time_to_iso(time: &str) -> u64 {
    let time_tmp: String = time.split(':').collect();
    time_tmp.parse::<u64>().unwrap()
}

fn display_calendar() {
    let mut cal = Calendar::new();
    cal.add_task(date_fmt("October 30th, 2015"), Task::new(time_to_iso("10:50"), String::from("Go Fish!")));
    cal.add_task(date_fmt("October 30th, 2015"), Task::new(time_to_iso("11:00"), String::from("Eat Fish!")));
    println!("{:?}", cal.check_tasks(date_fmt("October 30th, 2015")));
}

fn main() {
    display_calendar();
}
