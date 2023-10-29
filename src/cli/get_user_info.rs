use inquire::{Password, Text};

use crate::user::User;

pub fn get_user_info() -> User {
    let email_prompt = Text::new("Email:").prompt();

    let password_prompt = Password::new("Password:")
        .without_confirmation()
        .with_display_toggle_enabled()
        .with_help_message("Press Ctrl + R to reveal your password")
        .prompt();

    let mut user: User = User {
        email: "".to_string(),
        password: "".to_string(),
    };

    match email_prompt {
        Ok(email) => user.email = email,
        Err(_) => println!("An error happened when asking for your email, try again later."),
    }

    match password_prompt {
        Ok(password) => user.password = password,
        Err(_) => println!("An error happened when asking for your password, try again later."),
    }

    user
}
