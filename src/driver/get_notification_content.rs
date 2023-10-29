use thirtyfour::*;

use crate::notification::Notification;

pub async fn get_notification_content(driver: &WebDriver) -> Notification {
    let contents = driver.find_all(By::ClassName("cellcontent")).await.unwrap();

    let mut notification_content = Vec::new();

    for content in &contents {
        notification_content.push(content.text().await.unwrap());
    }

    let attachment: Option<WebElement> = {
        if contents.len() > 5 {
            // TODO: make support for many attachments
            Some(
                contents[5]
                    .find(By::ClassName("botaoDownload"))
                    .await
                    .unwrap(),
            )
        } else {
            None
        }
    };

    Notification {
        title: notification_content[0].to_owned(),
        body: Some(notification_content[1].to_owned()),
        date_received: notification_content[2].to_owned(),
        date_file: Some(notification_content[3].to_owned()),
        category: notification_content[4].to_owned(),
        attachment,
    }
}
