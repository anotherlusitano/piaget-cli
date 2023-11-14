use std::{
    env,
    fs::File,
    io::{Read, Write},
};

use crate::{cli::get_user_info::get_user_info, user::User};

#[allow(deprecated)]
pub fn verify_user() -> User {
    // WARNING: this only works on linux
    // and we use this bc I don't want to install another crate
    // see documentation here:
    // https://doc.rust-lang.org/std/env/fn.home_dir.html
    let infoestudante = env::home_dir().unwrap().join("infoestudante.txt");
    let user_file = infoestudante.is_file();

    if user_file {
        let mut file = File::open(infoestudante).expect("Couldn't open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't read the file");

        let email = contents.split('\n').collect::<Vec<_>>()[0];
        let password = contents.split('\n').collect::<Vec<_>>()[1];

        User {
            email: email.to_owned(),
            password: password.to_owned(),
        }
    } else {
        let user = get_user_info();
        let mut file = File::create(infoestudante).expect("Couldn't create the file");
        let user_info = format!("{}\n{}", user.email, user.password);

        writeln!(&mut file, "{}", user_info).expect("Couldn't write in the file");

        user
    }
}
