use std::process::Child;

use thirtyfour::WebDriver;

pub async fn kill_web_driver(driver: &WebDriver) {
    driver.to_owned().quit().await.unwrap();
}

pub fn kill_chrome_driver(mut chromdriver: Child) {
    chromdriver.kill().unwrap();
}
