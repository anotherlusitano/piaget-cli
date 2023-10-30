use core::fmt;

use thirtyfour::WebElement;

#[derive(Clone)]
pub struct Notification {
    pub title: String,
    pub body: Option<String>,
    pub date_received: String,
    pub date_file: Option<String>,
    pub category: String,
    pub attachments: Option<Vec<WebElement>>,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
