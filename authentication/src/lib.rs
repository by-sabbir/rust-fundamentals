use std::io::stdin;

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    if username == "admin" && password == "password" {
        return Some(LoginAction::Granted(LoginRole::Admin));
    } else if username == "bob" && password == "password" {
        return Some(LoginAction::Granted(LoginRole::User));
    } else if username != "bob" && username != "admin" {
        return None;
    } else {
        return Some(LoginAction::Denied);
    }
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[cfg(test)]
mod tests;
