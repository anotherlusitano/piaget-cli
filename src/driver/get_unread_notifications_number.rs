use thirtyfour::prelude::*;

pub async fn get_unread_notifications_number(driver: &WebDriver) {
    let notifications_dash = driver
        .query(By::Id("dashWidgetContentW_AL_01"))
        .first()
        .await
        .unwrap();

    let notif_element = notifications_dash.query(By::Css("strong"));

    let has_notifications: bool = notif_element.exists().await.unwrap();

    if !has_notifications {
        println!("0");
        return;
    }

    let notif_text: String = notif_element.first().await.unwrap().text().await.unwrap();

    let notif_num = notif_text.split_whitespace().collect::<Vec<&str>>()[1];

    println!("{notif_num}");
}
