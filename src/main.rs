use crate::utils::Solve;
use std::env;
use chrono::{Datelike, Local};

mod utils;
mod geometry;
mod y2024;
mod y2015;
mod y2016;

use std::time::{Duration, Instant};

fn timeit<F>(mut func: F, n_iterations: u32) -> Result<Duration, String>
where
    F: FnMut() -> Result<String, String>,
{
    let start = Instant::now();
    for _ in 0..n_iterations {
        func()?;
    }
    Ok(start.elapsed()/n_iterations)
}

fn format_duration(duration: Duration) -> String{
    let total_seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0;
    if total_seconds >= 1.0 {
        format!("{:.1}s", total_seconds)
    } else if duration.as_millis() >= 1 {
        let total_millis = (duration.as_micros() as f64)/1_000.0;
        format!("{:.1}ms", total_millis)
    } else {
        let total_micros = (duration.as_nanos() as f64)/1_000.0;
        format!("{:.1}µs", total_micros)
    }
}

fn run(a: &mut Box<dyn Solve>, n_iterations: u32, test_mode: bool, bruteforce: bool) {
    println!("{}", "-".repeat(50));
    println!(":::Day {}:::", a.get_label().day);
    if bruteforce {
        a.apply_bruteforce();
    }

    if let Err(_) = a.read_input(test_mode) {
        println!("Cannot read puzzle input");
        return;
    }

    if let Err(msg) = a.info() {println!("{}", msg)};

    // Define the method closures with explicit types
    let methods: [(&str, Box<dyn Fn(&mut Box<dyn Solve>, bool) -> Result<String, String>>); 2] = [
        (
            "Part 1",
            Box::new(|a: &mut Box<dyn Solve>, test_mode| a.compute_part1_answer(test_mode))
        ),
        (
            "Part 2",
            Box::new(|a: &mut Box<dyn Solve>, test_mode| a.compute_part2_answer(test_mode))
        ),
    ];

    // Iterate over the methods
    for (part_name, method) in methods.iter() {
        if n_iterations > 0 {
            let d = timeit(|| { method(a, test_mode) }, n_iterations);
            if let Ok(d) = d {
                println!("Time taken {}: {}", part_name, format_duration(d));
            }
        }else{
            match method(a, test_mode) {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(msg) => {
                    {println!("Error: {}", msg);}
                }
            }
        }
    }
    println!("{}", "\n");
}

fn main() {
    let current_date = Local::now().date_naive();
    let mut year = current_date.year() as u16;
    let mut n_iterations = 0;
    let mut test_mode = false;
    let mut first_day: u8 = 1;
    let mut last_day: u8 = 25;
    let mut bruteforce: bool = false;

    let args: Vec<String> = env::args().collect();
    let mut itr = args.iter().skip(1);

    while let (Some(key), Some(value)) = (itr.next(), itr.next()) {
        match key.as_str() {
            "-t" => test_mode = value.parse::<bool>().unwrap_or(test_mode),
            "-b" => bruteforce = value.parse::<bool>().unwrap_or(bruteforce),
            "-i" => n_iterations = value.parse::<u32>().unwrap_or(n_iterations),
            "-fd" => first_day = value.parse::<u8>().unwrap_or(first_day),
            "-ld" => last_day = value.parse::<u8>().unwrap_or(last_day),
            "-y" => year = value.parse::<u16>().unwrap_or(year),
            _ => {}
        }
    }
    last_day = [first_day, last_day].into_iter().max().unwrap();

    let mut solutions =
    match year{
        2015 => y2015::collect_solutions(),
        2016 => y2016::collect_solutions(),
        2024 => y2024::collect_solutions(),
        _ => unreachable!()
    };

    for day in first_day..=last_day {
        if let Some(a) = solutions.get_mut(&day) {
            run(a, n_iterations, test_mode, bruteforce);
        }
    }
}
