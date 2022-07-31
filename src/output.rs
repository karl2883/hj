use colored::Colorize;

pub fn print_debug(msg: &str) {
    println!("{} {}",
        format!("Debug:").green().bold(),
        format!("{}", msg).yellow(),
    );
}

pub fn print_error(msg: &str) {
    println!("{} {}",
        format!("Error:").red().bold(),
        format!("{}", msg).bold(),
    );
}
