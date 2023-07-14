use chrono::prelude::*;
use dialoguer::Input;

pub struct Menu;
impl Menu {
    pub fn show_input(&self) -> String {
        let now_year = Local::now().year().to_string();
        let now_month = Local::now().month().to_string();
        let now_day = Local::now().day().to_string();
        let now_date = format!("{}-{}-{}", now_year, now_month, now_day);
        let input: String = Input::new()
            .with_prompt("登録日：")
            .with_initial_text(now_date)
            .interact_text()
            .unwrap();

        return input;
    }
}

fn main() {
    let input = Menu.show_input();
    println!("{}", input);
}
