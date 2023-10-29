use crate::driver::{
    kill_drivers::{kill_chrome_driver, kill_web_driver},
    start_drivers::{start_chrome_driver, start_web_driver},
};
use clap::Command;
use cli::display_notification::display_notification;
use driver::{
    get_notification_content::get_notification_content, get_notifications::get_notifications,
};
use notification_filter::NotificationFilter;
use std::{thread::sleep, time::Duration};
use validation::{verify_login::verify_login, verify_user::verify_user};

mod cli;
mod driver;
pub mod notification;
pub mod notification_filter;
pub mod user;
mod validation;

#[tokio::main]
async fn main() {
    let matches = Command::new("piaget-cli")
        .about("CLI tool to read notifications from Piaget")
        .subcommand_required(true)
        .subcommand(
            Command::new("read")
                .about("Search for your read notifications")
                .short_flag('r'),
        )
        .subcommand(
            Command::new("unread")
                .about("Search for your unread notifications")
                .short_flag('u'),
        )
        .get_matches();

    let user = verify_user();

    let chromedriver = start_chrome_driver();
    // we need to wait the chromedriver to start
    sleep(Duration::from_millis(100));

    let driver = start_web_driver().await;

    verify_login(&driver, user).await.unwrap();

    match matches.subcommand() {
        Some(("read", _)) => {
            let filter = NotificationFilter {
                index: 1,
                tag: "td".to_owned(),
            };

            get_notifications(&driver, filter).await;
        }
        Some(("unread", _)) => {
            let filter = NotificationFilter {
                index: 2,
                tag: "b".to_owned(),
            };

            get_notifications(&driver, filter).await;
        }
        _ => println!("No Choice?"),
    }

    let notification = get_notification_content(&driver).await;

    display_notification(&driver, notification).await;

    kill_web_driver(&driver).await;
    kill_chrome_driver(chromedriver);
}
