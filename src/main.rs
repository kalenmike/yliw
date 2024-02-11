use std:: {
    io::{self, Write},
    thread,
    time::Duration,
    fs,
};
use colored::Colorize;
use chrono::{NaiveDate, Local, ParseError};
use indicatif::{ProgressBar, ProgressStyle};
use dirs;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct UserConfig {
    birthday: Option<String>,
    show_weeks: Option<bool>,
}

/// Prints a visual representation of life in weeks.
///
/// This function prints a grid where each cell represents one week of life. 
/// The weeks that have been lived are marked in green, and the remaining weeks are marked in cyan.
///
/// # Arguments
///
/// * `lived_weeks` - The number of weeks lived. This will determine how many cells are marked in green.
///
/// # Examples
///
/// ```
/// // Assuming the use of the colored and time crates
/// use yliw::print_life_in_weeks;
///
/// // Print a representation for a 25-year-old person
/// // (25 years * 52 weeks/year)
/// print_life_in_weeks(25 * 52);
/// ```
///
/// # Panics
///
/// This function will not panic under normal circumstances.
///
/// # Errors
///
/// This function does not return errors. However, it sleeps for 20ms after printing each row,
/// which could slightly delay program execution.
///
/// # Notes
///
/// This function is primarily for visual representation and does not return any value.
fn print_life_in_weeks(lived_weeks: usize){
    let rows = 30;
    let columns = 156;
    let pattern = "=";
    
    println!("\n{}\n", "Your life in weeks:".bold());
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

/// Parses a string into a `NaiveDate`.
///
/// This function attempts to parse a given string representing a date in the format `DD-MM-YYYY`.
/// If successful, it returns a `NaiveDate` object; otherwise, it returns a `ParseError`.
///
/// # Arguments
///
/// * `input` - A string slice that holds the date to be parsed.
///
/// # Returns
///
/// This function returns a `Result<NaiveDate, ParseError>`. On success, it provides the parsed `NaiveDate`.
/// On failure, it provides a `ParseError` indicating the reason for the failure.
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate, ParseError};
/// use yliw::parse_date;
///
/// let date_str = "27-04-2021";
/// match parse_date(date_str) {
///     Ok(date) => println!("Parsed date: {}", date),
///     Err(e) => println!("Error parsing date: {}", e),
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the input string does not conform to the `DD-MM-YYYY` format,
/// or if the date is not a valid calendar date (e.g., "31-02-2021").
fn parse_date(input: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(input, "%d-%m-%Y")
}

/// Display a welcome message.
/// 
/// This function prints a welcome message to the console, emphasizing it with green color.
/// 
/// # Examples
/// 
/// ```
/// use colored::Colorize;
/// display_welcome_message();
/// ```
fn display_welcome_message(){
    let welcome = "Welcome to Life!";
    println!("{}", welcome.green());
    println!();
}

/// Prompt the user to input their birthday and return it as a string.
///
/// This function prompts the user to input their birthday in the format "DD-MM-YYYY".
/// It reads the input from the standard input and returns the trimmed string.
///
/// # Examples
///
/// ```
/// assert_eq!(get_user_birthday(), "01-01-1990");
/// ```
fn get_user_birthday() -> String {
    let mut birthday = String::new();
    print!("What is your birthday (DD-MM-YYYY)? ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut birthday)
        .expect("Failed to read input.");

    birthday.trim().to_string()
}

/// Calculate the age in days based on the provided birthday.
///
/// This function takes a string representing a birthday in the format "DD-MM-YYYY".
/// It parses the input birthday string and calculates the age in days relative to the current date.
/// If the parsing fails, it prints an error message and exits the program.
///
/// # Arguments
///
/// * `birthday` - A string slice representing the birthday in the format "DD-MM-YYYY".
///
/// # Returns
///
/// The age in days as an integer (`i32`) if parsing is successful.
///
/// # Panics
///
/// This function will panic if parsing of the input date fails.
///
/// # Examples
///
/// ```
/// assert_eq!(get_age_in_days("01-01-1990"), 12053);
/// ```
fn get_age_in_days(birthday: &str) -> i32{
    let result = match parse_date(&birthday.trim()){
        Ok(input_date) => {
            let current_date = Local::now().naive_local().date();
            let duration = current_date.signed_duration_since(input_date);
            duration.num_days() as i32
        },
        Err(_) => {
            println!("Unable to parse the date. Please use DD-MM-YYYY.");
            std::process::exit(1);
        },
    };
    result
}

/// Create a progress bar with a specified total.
///
/// This function creates and configures a progress bar with the specified total steps.
/// It sets a custom style for the progress bar, including the template and progress characters.
///
/// # Arguments
///
/// * `total` - The total number of steps for the progress bar.
///
/// # Returns
///
/// A `ProgressBar` instance configured with the specified total and custom style.
///
/// # Panics
///
/// This function will panic if there is an error parsing the progress bar template.
///
/// # Examples
///
/// ```
/// let progress_bar = create_progress_bar(100);
/// progress_bar.set_message("Processing");
/// progress_bar.inc(10);
/// ```
fn create_progress_bar(total: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(total);
    let style = ProgressStyle::default_bar()
                            .template("[{bar:70.green/cyan}]")
                            .expect("Error parsing progress bar template")
                            .progress_chars("=>-");
    progress_bar.set_style(style);
    progress_bar
}

/// Display a progress bar representing the passage of years.
///
/// This function updates a given progress bar to reflect the passage of years up to the specified age.
/// It sets the position of the progress bar to represent each year and pauses briefly to simulate progress.
///
/// # Arguments
///
/// * `progress_bar` - A mutable reference to a `ProgressBar` instance to be updated.
/// * `age_in_years` - The age in years to represent with the progress bar.
///
/// # Examples
///
/// ```
/// use indicatif::ProgressBar;
/// let mut pb = ProgressBar::new(10);
/// display_progress_bar(&mut pb, 5);
/// ```
fn display_progress_bar(progress_bar :&mut  ProgressBar, age_in_years: i32) {
    for year in 0..=age_in_years {
        progress_bar.set_position(year as u64);
        thread::sleep(Duration::from_millis(20));
    }
    println!();
}

/// Display a summary message based on expected and remaining days.
///
/// This function calculates and displays a summary message based on the expected
/// and remaining days. It calculates the remaining years, weeks, and the completion
/// percentage of the expected days. Then it formats and prints a message indicating
/// the remaining time in terms of birthdays, weekends, and breakfasts.
///
/// # Arguments
///
/// * `expected_days` - The total number of expected days.
/// * `remaining_days` - The number of remaining days.
///
/// # Examples
///
/// ```
/// display_summary_message(36500, 18250);
/// ```
fn display_summary_message(expected_days: i32, age_in_days: i32){
    let remaining_days = expected_days - age_in_days;
    let remaining_years = remaining_days / 365;
    let remaining_weeks = remaining_years * 52;
    let completion_percent = (age_in_days as f64 / expected_days as f64) * 100.0;

     let message = format!(
        "\n\n{}\n\nLooking ahead, here's what's still in store for you:\n\n\
        - Celebrate: {} more birthdays\n\
        - Relax: {} more weekends\n\
        - Enjoy: {} more breakfasts\n",
        format!("Your life is {:.2}% complete!", completion_percent).green().bold(),
        format!("{} wonderful", remaining_years).yellow(),
        format!("{} relaxing", remaining_weeks).cyan(),
        format!("{} delicious", remaining_days).magenta()
    );

    println!("{}", message);
}

/// Main function to run the life progress program.
///
/// This function orchestrates the execution of the life progress program.
/// It calculates the user's age in days, weeks, and years based on their birthday,
/// displays a welcome message, creates a progress bar representing the user's life,
/// and displays a summary message indicating the remaining time and progress.
/// Finally, it prints the user's life in weeks using a simple chart.
///
/// # Examples
///
/// ```
/// main();
/// ```
fn main() {
    let expected_years = 90; // Assume humans live 90 years
    let expected_days = expected_years * 365;

    display_welcome_message();

    let mut config_dir = dirs::config_dir().expect("Failed to locate user's config directory");
    config_dir.push("yliw");
    config_dir.push("config.toml");

    let mut user_config: UserConfig;
    if let Ok(toml_content) = fs::read_to_string(&config_dir){
        user_config = toml::from_str(&toml_content)
            .expect("Failed to parse config file");
    }else{
        user_config = UserConfig { birthday: None, show_weeks: Some(true) };
    }

    if user_config.birthday.is_none() {
        let birthday = get_user_birthday();
        user_config.birthday = Some(birthday);
    }

    let age_in_days;
    if let Some(birthday) = &user_config.birthday {
        age_in_days = get_age_in_days(birthday);
    } else {
        println!("Birthday not found.");
        std::process::exit(1);
    }
    
    let age_in_weeks = age_in_days as f64 / 7.0;
    let age_in_years = age_in_weeks as i32 / 52;

    let mut life_progress_bar = create_progress_bar(expected_years as u64);
    
    println!("{}", format!("You are {} years old!", age_in_years).italic());
    println!();

    display_progress_bar(&mut life_progress_bar, age_in_years);
    display_summary_message(expected_days, age_in_days);
    
    if user_config.show_weeks.unwrap_or(true){
        print_life_in_weeks(age_in_weeks as usize);
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_valid(){
        let date_str = "21-12-2021"; // Valid date
        let result = parse_date(date_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NaiveDate::from_ymd_opt(2021, 12, 21).unwrap());
    }

    #[test]
    fn test_parse_date_invalid(){
        let date_str = "21-13-2021"; // Invalid date
        let result = parse_date(date_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_date_format(){
        let date_str = "2021-12-21"; // Invalid Format
        let result = parse_date(date_str);
        assert!(result.is_err());
    }
}
