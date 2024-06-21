use authentication::{greet_user, login, read_line, LoginAction, LoginRole};

fn main() {
    let mut retries = 0;
    loop {
        println!("username: ");
        let username = read_line();
        println!("password: ");
        let password = read_line();

        let login_action = login(&username, &password);
        match login_action {
            Some(login_action) => match login_action {
                LoginAction::Granted(LoginRole::Admin) => {
                    println!("Admin Login\t{}", greet_user(&username));
                    break;
                }
                LoginAction::Granted(LoginRole::User) => {
                    println!("User Login\t{}", greet_user(&username));
                    break;
                }
                LoginAction::Denied => {
                    retries += 1;
                    if retries >= 3 {
                        println!("Too Many Attempts.");
                        break;
                    }
                    println!("Access Denied; Retries; {retries}/3");
                }
            },
            None => {
                println!("New User");
                break;
            }
        }
    }
}
