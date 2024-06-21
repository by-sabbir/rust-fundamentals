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
                    println!("Admin Login\n{}", greet_user(&username));
                    break;
                }
                LoginAction::Granted(LoginRole::User) => {
                    println!("User Login\n{}", greet_user(&username));
                    break;
                }
                LoginAction::Denied => {
                    retries += 1;
                    if retries >= 3 {
                        println!("too many attempts");
                        break;
                    }
                    println!("Login Denied; retry attemps: {retries}/3");
                }
            },
            None => {
                println!("new user");
                break;
            }
        }
    }
}
