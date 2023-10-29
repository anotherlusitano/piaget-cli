use std::error::Error;

use thirtyfour::prelude::*;

use crate::user::User;

pub async fn verify_login(driver: &WebDriver, user: User) -> Result<(), Box<dyn Error>> {
    let url = "https://inforestudante.ipiaget.org";

    driver.goto(url).await?;

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
