use std::{error::Error, process::exit};

use thirtyfour::prelude::*;

use crate::user::User;

pub async fn verify_login(driver: &WebDriver, user: User) -> Result<(), Box<dyn Error>> {
    let url = "https://inforestudante.ipiaget.org";

    // I was tired of getting a long error message
    // every time I didn't have wifi
    if (driver.goto(url).await).is_err() {
        eprintln!("?");
        exit(1)
    }

    let username_input = driver.find(By::Id("username")).await?;

    username_input.send_keys(user.email).await?;

    let password_input = driver.find(By::Id("password1")).await?;

    password_input.send_keys(user.password).await?;

    driver
        .query(By::ClassName("button"))
        .first()
        .await?
        .click()
        .await?;

    Ok(())
}
