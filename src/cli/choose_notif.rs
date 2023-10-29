use inquire::{InquireError, Select};

use crate::notification::Notification;

pub fn choose_notif(notifications: Vec<Notification>) -> Result<usize, ()> {
    let options: Vec<String> = notifications
        .iter()
        .enumerate()
        .map(|(index, item)| {
            format!(
                "{} - {} | {} | {}",
                index + 1,
                item.category,
                item.title,
                item.date_received
            )
        })
        .collect();

    let help_msg = format!("Total of notifications: {}", &options.len());

    let ans: Result<String, InquireError> =
        Select::new("Select the notification that you want to read:", options)
            .with_help_message(&help_msg)
            .with_page_size(25)
            .prompt();

    match ans {
        Ok(choice) => {
            let mut media_index = choice.split_whitespace();

            let index: usize = media_index.next().unwrap().parse::<usize>().unwrap();

            //we need the -1 because the vec start at zero
            Ok(index - 1)
        }
        Err(_) => Err(println!("There was an error, please try again")),
    }
}
