use thirtyfour::prelude::*;

pub async fn get_unread_notifications_number(driver: &WebDriver) {
    let notifications_dash = driver
        .query(By::Id("dashWidgetContentW_AL_01"))
        .first()
        .await
        .unwrap();

    let notif_text: String = notifications_dash
        .query(By::Css("strong"))
        .first()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let notif_num = notif_text.split_whitespace().collect::<Vec<&str>>()[1];

    println!("{notif_num}");
}
