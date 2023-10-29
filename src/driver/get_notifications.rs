use thirtyfour::prelude::*;

use crate::{
    cli::choose_notif::choose_notif, notification::Notification,
    notification_filter::NotificationFilter,
};

pub async fn get_notifications(driver: &WebDriver, filter: NotificationFilter) {
    let notifications_btn = driver
        .query(By::ClassName("menu_10"))
        .first()
        .await
        .unwrap();

    // we execute a js script to be redirect to the correct page
    driver
        .execute(
            r#"
            arguments[0].click();
            "#,
            vec![notifications_btn.to_json().unwrap()],
        )
        .await
        .expect("Error: Can't click on the notifications");

    let select_filter_elm = driver.find(By::Id("tipoCaixa")).await.unwrap();

    select_filter_elm.find_all(By::Tag("option")).await.unwrap()[filter.index]
        .click()
        .await
        .unwrap();

    let notifications = driver.query(By::Id("notificacoes")).exists().await.unwrap();

    if notifications {
        let notifications_container = driver.find(By::Id("notificacoes")).await.unwrap();

        let notifications_body = notifications_container
            .find(By::Css("tbody"))
            .await
            .unwrap();

        let notifications = notifications_body.find_all(By::Css("tr")).await.unwrap();

        let mut notifications_list = Vec::new();

        for notification in notifications {
            let category_label = notification.find(By::Css("label")).await.unwrap();
            let title_and_date = notification.find_all(By::Css(&filter.tag)).await.unwrap();

            let notification = Notification {
                category: category_label.text().await.unwrap(),
                title: title_and_date[0].text().await.unwrap(),
                date_received: title_and_date[1].text().await.unwrap(),
                date_file: None,
                body: None,
                attachment: None,
            };

            notifications_list.push(notification);
        }

        let notification_index: usize = choose_notif(notifications_list).unwrap();
        let notifications_btn = notifications_body.find_all(By::Css("a")).await.unwrap();

        notifications_btn[notification_index].click().await.unwrap();
    } else {
        println!("You don't have any notifications!");
    }
}
