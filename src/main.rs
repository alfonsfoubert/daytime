use chrono::Local;

fn main() {
    let formatted_date = get_current_date_formatted();
    println!("{}", formatted_date);
}

fn get_current_date_formatted() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

