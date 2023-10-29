use std::{env, thread::sleep, time::Duration};

use crate::notification::Notification;
use termimad::*;
use thirtyfour::WebDriver;

pub async fn display_notification(driver: &WebDriver, notification: Notification) {
    let content: String = format!(
        "
----
# {}

{}

**Data de Receção:** {}

**Data de Arquivo:** {}

**Tipo de Mensagem:** {}
    ",
        notification.title,
        notification.body.unwrap(),
        notification.date_received,
        notification.date_file.unwrap(),
        notification.category
    );

    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(rgb(255, 187, 0));
    skin.print_text(&content);

    if notification.attachment.is_some() {
        let attachment_text = notification
            .attachment
            .as_ref()
            .unwrap()
            .clone()
            .text()
            .await
            .unwrap();
        let attachment_link = notification.attachment.unwrap().attr("href").await.unwrap();

        skin.print_text(format!("**Anexo(s):** {}\n", &attachment_text).as_str());
        print_text("----");

        let yes_respond_idk = format!("Yes, I'd like to download **{}**.", attachment_text);

        let url = format!(
            "https://inforestudante.ipiaget.org/nonio/notificacoes/{}",
            attachment_link.unwrap()
        );
        ask!(&skin, "Do you want to download this attachment?", {
            ('y', yes_respond_idk) => {
                driver.goto(url).await.unwrap();

                // TODO: make this compatible with windows idk
                let home_dir: String = env::var("HOME").unwrap();
                let download_dir: String = format!("{}/Downloads/", home_dir);

                // BUG: this don't download the whole name of the file
                for file in std::path::Path::new(&download_dir).read_dir().unwrap() {
                    let file_path = file.unwrap().path();
                    while file_path.with_extension("crdownload") == file_path && file_path.exists() {
                        sleep(Duration::from_millis(500));
                    }
                }
                println!("Download completed!");
            }
            ('n', "No.") => {
            }
        });
    } else {
        print_text("----");
    }
}
