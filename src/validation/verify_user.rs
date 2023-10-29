use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::{cli::get_user_info::get_user_info, user::User};

pub fn verify_user() -> User {
    // TODO: change the location of the file to somewhere else
    let user_file = Path::new("infoestudante.txt").is_file();

    if user_file {
        let mut file = File::open("infoestudante.txt").expect("Couldn't open the file");
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
        let mut file = File::create("infoestudante.txt").expect("Couldn't create the file");
        let user_info = format!("{}\n{}", user.email, user.password);

        writeln!(&mut file, "{}", user_info).expect("Couldn't write in the file");

        user
    }
}
