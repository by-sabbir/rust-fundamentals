use std::{collections::HashMap, fs, io::stdin, path::Path};

use serde::{Deserialize, Serialize};

// temporary user storage
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(name: &str, pass: &str, role: LoginRole) -> User {
        Self {
            name: name.to_lowercase(),
            password: pass.to_string(),
            role,
        }
    }
}

fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();

    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "bob".to_string(),
        User::new("bob", "password", LoginRole::User),
    );

    users
}

fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let user_json = fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&user_json).unwrap();

        users
    } else {
        let users = get_default_users();
        let user_json = serde_json::to_string_pretty(&users).unwrap();
        let _ = fs::write(users_path, user_json);
        users
    }
}

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();

    if let Some(user) = users.get(username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    return None;
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
