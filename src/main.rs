use chrono::prelude::*;
use std::error::Error;

const SESC_URL: &'static str = "https://www.sescsp.org.br/cardapio-semanal-sesc-santo-andre-2/";
const NUMBER_OF_MENU_LINES: u8 = 7;

fn main() -> Result<(), Box<dyn Error>>{
    let now = Local::now();

    let menu: Vec<String> = get_menu(now)?;

    print_menu(&menu);

    Ok(())
}

fn print_menu(menu: &Vec<String>) {
    for item in menu {
        println!();
        println!("{}", item);
    }
    println!();
}

fn format_menu(menu: &Vec<String>) -> Vec<String> {
    let mut formatted_menu: Vec<String> = Vec::new();

    for item in menu {
        let mut formatted_item = item
                                            .replace("<strong>", "")
                                            .replace("</strong>", "");

        if contains_digit(&formatted_item) {
            formatted_item.insert(0, ' ');
        }

        formatted_menu.push(formatted_item);
    }

    formatted_menu
}

fn contains_digit(s: &str) -> bool {
    s.chars().any(|c| c.is_numeric())
}

fn get_menu(date: DateTime<Local>) -> Result<Vec<String>, Box<dyn Error>> {
    if date.date_naive().weekday() == chrono::Weekday::Mon {
        return Err("SESC Fechado!!!! Segunda-feira Bebê!".into());
    }

    let day_and_month: String = date.format("%-d/%-m").to_string();

    let response: String = reqwest::blocking::get(SESC_URL)?.text()?;

    let document: scraper::Html = scraper::Html::parse_document(&response);

    let selector: scraper::Selector = scraper::Selector::parse("p").unwrap();

    // Control variables
    let mut trigger: bool = false;
    let mut counter: u8 = 0;

    // Menu text
    let mut menu: Vec<String> = Vec::with_capacity(NUMBER_OF_MENU_LINES as usize);

    for element in document.select(&selector) {
        if counter == NUMBER_OF_MENU_LINES {
            break;
        }
        if element.inner_html().contains(&day_and_month) {
            trigger = true;
        }
        if trigger {
            counter += 1;
            menu.push(element.inner_html());
        }
    }

    // Format each menu item text
    menu = format_menu(&menu);

    Ok(menu)
}
