use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;
use chrono::{NaiveDate, Local, ParseError};

fn print_life_in_weeks(lived_weeks: usize){
    let rows = 30;
    let columns = 156;
    let pattern = "=";

    for row in 0..rows {
        for col in 0..columns {

            let total_weeks_so_far = row * columns + col;
            if total_weeks_so_far < lived_weeks {
                print!("{}", pattern.green());
            }else {
                print!("{}", pattern.cyan());
            }
        }
        println!(); 
        thread::sleep(Duration::from_millis(20));
    }
}

fn parse_date(input: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(input, "%d-%m-%Y")
}

fn main() {
    let mut birthday = String::new();

    let welcome = "Welcome to Life!";
    println!("{}", welcome.green());
    println!();

    print!("What is your birthday (DD-MM-YYYY)? ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut birthday)
        .expect("Failed to read input.");

    let age_in_days = match parse_date(&birthday.trim()){
        Ok(input_date) => {
            let current_date = Local::now().naive_local().date();
            let duration = current_date.signed_duration_since(input_date);
            duration.num_days()
        },
        Err(_) => {
            println!("Unable to parse the date. Please use DD-MM-YYYY.");
            return
        },
    };

    let expected_years = 90;
    let expected_weeks = (90 * 52) as f64;

    let age_in_weeks = age_in_days as f64 / 7.0;
    let age_in_years = age_in_weeks as i32 / 52;

    let remaining_years = expected_years - age_in_years;
    let remaining_weeks = (expected_weeks - age_in_weeks) as i32;
    let life_completion_percent = (age_in_weeks as f64 / expected_weeks) * 100.0;
    
    let life_progress = ProgressBar::new(expected_years as u64);
    let style = ProgressStyle::default_bar()
                            .template("[{bar:70.green/cyan}]")
                            .expect("Error parsing progress bar template")
                            .progress_chars("=>-");
    life_progress.set_style(style);

    println!("{}", format!("You are {} years old!", age_in_years).italic());
    println!();

    for i in 0..=age_in_years {
        life_progress.set_position(i as u64);
        thread::sleep(Duration::from_millis(20));
    }
    println!();

    let message = format!(
        "\n{}\n\nLooking ahead, here's what's still in store for you:\n\n\
        - Celebrate: {} more birthdays\n\
        - Relax: {} more weekends\n\
        - Enjoy: {} more breakfasts\n",
        format!("Your life is {:.2}% complete!", life_completion_percent).green().bold(),
        format!("{} wonderful", remaining_years).yellow(),
        format!("{} relaxing", remaining_weeks).cyan(),
        format!("{} delicious", remaining_years * 365).magenta()
    );

    println!("{}", message);
    println!();

    print_life_in_weeks(age_in_weeks as usize);
}



// Get the birthday
// Get the current date
// 4680 is standard weeks in a life



