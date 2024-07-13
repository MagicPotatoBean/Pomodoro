use std::{
    ops::{Div, Mul},
    process, thread,
    time::{Duration, Instant},
};
const ESC: char = 27 as char;
fn main() {
    ctrlc::set_handler(|| {
        print!("{ESC}[?25h");
        println!("\nGoodbye!");
        process::exit(-1);
    })
    .unwrap();
    let mut formatted_args = Vec::new();
    let mut args = std::env::args().peekable();
    args.next();
    let should_loop = args.peek().unwrap() == "loop";
    if should_loop {
        args.next();
    }
    loop {
        if let Some(duration) = args.next() {
            let duration: u64 = parse_time(&duration);
            let name = args.next().expect("No label for provided time");
            formatted_args.push((name, Duration::from_secs(duration)));
        } else {
            break;
        }
    }
    loop {
        for (msg, dur) in formatted_args.iter() {
            print!("\x07");
            run(*dur, msg);
        }
        if !should_loop {
            break;
        }
    }
}
fn parse_time(input: &str) -> u64 {
    match input.chars().last().expect("Invalid time") {
        's' => {
            let val: f64 = input[0..(input.len() - 1)].parse().unwrap();
            val
        }
        'm' => {
            let val: f64 = input[0..(input.len() - 1)].parse().unwrap();
            val * 60_f64
        }
        'h' => {
            let val: f64 = input[0..(input.len() - 1)].parse().unwrap();
            val * 60_f64 * 60_f64
        }
        _ if input.chars().all(|chr| chr.is_numeric() || chr == '.') => {
            let val: f64 = input.parse().unwrap();
            val * 60_f64
        }
        _ => {
            panic!("test")
        }
    }
    .floor() as u64
}
fn run(duration: Duration, message: &str) {
    const FREQUENCY: u64 = 100; // in Hz
    let now = Instant::now();
    print!("{ESC}[?25l\n\n\n");
    loop {
        let elapsed = now.elapsed();
        print!("{ESC}[13D");
        print!("{ESC}[3A");
        if elapsed > duration {
            break;
        }
        print_ui(duration, elapsed, message);
        thread::sleep(Duration::from_millis(1000.div(FREQUENCY)));
    }
    print!("{ESC}[?25h");
}
fn print_ui(total_time: Duration, current_time: Duration, message: &str) {
    assert!(current_time.le(&total_time));
    let total_time_chrono = chrono::Duration::from_std(total_time).unwrap();
    let current_time_chrono = chrono::Duration::from_std(current_time).unwrap();
    let time_str = format!(
        "{:0>2}:{:0>2} | {:0>2}:{:0>2}",
        current_time_chrono.num_minutes(),
        current_time_chrono.num_seconds() - current_time_chrono.num_minutes() * 60,
        total_time_chrono.num_minutes(),
        total_time_chrono.num_seconds() - total_time_chrono.num_minutes() * 60
    );
    let line_len = time_str.len().max(message.len());
    let chars_visible = current_time
        .div_duration_f64(total_time)
        .mul(line_len as f64);
    println!(
        "{}{time_str}",
        " ".repeat((line_len - time_str.len()).div(2))
    );
    print!("{}{message}", " ".repeat((line_len - message.len()).div(2)));
    println!("{}", " ".repeat(line_len - message.len()));
    let mut progress_bar = "█".repeat(chars_visible.floor() as usize);
    let remainder = chars_visible - (chars_visible.floor());
    if remainder > 0.875 {
        progress_bar.push('▉')
    } else if remainder > 0.750 {
        progress_bar.push('▉')
    } else if remainder > 0.625 {
        progress_bar.push('▊')
    } else if remainder > 0.5 {
        progress_bar.push('▋')
    } else if remainder > 0.375 {
        progress_bar.push('▌')
    } else if remainder > 0.25 {
        progress_bar.push('▍')
    } else if remainder > 0.125 {
        progress_bar.push('▎')
    } else if remainder > 0.0 {
        progress_bar.push('▏')
    }
    progress_bar.push_str(&" ".repeat(line_len - progress_bar.chars().count()));
    println!("{progress_bar}");
}
