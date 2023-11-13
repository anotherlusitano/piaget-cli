use std::{
    env,
    process::{Child, Command},
};

use thirtyfour::{DesiredCapabilities, WebDriver};

pub async fn start_web_driver() -> WebDriver {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().unwrap();

    // TODO: make this compatible with windows idk
    let home_dir: String = env::var("HOME").unwrap();
    let download_dir: String = format!("{}/Downloads/", home_dir);

    let chrome_prefs = serde_json::json!({
      "download.default_directory": download_dir,
      "download.prompt_for_download": false
    });

    caps.add_chrome_option("prefs", chrome_prefs).unwrap();

    WebDriver::new("http://localhost:9515", caps).await.unwrap()
}

pub fn start_chrome_driver() -> Child {
    let chromedriver;
    {
        chromedriver = Command::new("chromedriver")
            .stdout(std::process::Stdio::null())
            .spawn()
            .unwrap();
    }

    chromedriver
}
