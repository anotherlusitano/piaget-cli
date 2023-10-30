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

    if notification.attachments.is_some() {
        skin.print_text("**Anexo(s):**");

        for attachment in notification.attachments.unwrap() {
            let question = format!(
                "Do you want to download **{}**?",
                attachment.text().await.unwrap()
            );

            let url = format!(
                "https://inforestudante.ipiaget.org/nonio/notificacoes/{}",
                attachment.attr("href").await.unwrap().unwrap()
            );
            ask!(&skin, question, {
                ('y', "Yes.") => {
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
                    skin.print_text("**Download completed!**");
                }
                ('n', "No.") => {
                }
            });
            print_text("\n");
        }
    }
    print_text("----");
}
