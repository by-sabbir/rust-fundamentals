use std::{collections::HashMap, fs, path::Path};

use authentication::{get_users, hash_password, LoginRole, User};
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[command()]
struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a new user.
    Add {
        /// User's login name
        name: String,
        /// User's password
        password: String,

        /// Optional: promote user to admin
        #[arg(long, short)]
        admin: Option<bool>,
    },
    /// Delete a user.
    Delete {
        /// User's login name
        username: String,
    },
    /// Change User's Profile
    Update {
        /// User's name
        name: String,
        /// Optional - update admin status
        #[arg(long, short)]
        admin: Option<bool>,
        /// Optional - update user's password
        #[arg(long, short)]
        password: Option<String>,
    },
}

fn save_user(users: HashMap<String, User>) {
    let users_path = Path::new("../users.json");

    let users_json = serde_json::to_string_pretty(&users).unwrap();

    let _ = fs::write(users_path, users_json);
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Password");
    println!("{:-<40}", "");

    let users = get_users();

    users
        .into_iter()
        .for_each(|(name, user)| println!("{:<20}{:20?}", name, user.password))
}

fn add_user(username: String, password: String, admin: bool) {
    let user = if admin {
        User::new(&username, &password, LoginRole::Admin)
    } else {
        User::new(&username, &password, LoginRole::User)
    };

    let mut users = get_users();
    users.insert(username, user);

    save_user(users);
}

fn delete_user(username: String) {
    let mut users = get_users();

    users.remove(&username);

    save_user(users);
}

fn update_user(username: String, password: Option<String>, admin: Option<bool>) {
    let mut users = get_users();

    let user = users.get_mut(&username).unwrap();
    let role = if admin.unwrap_or(false) {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    user.role = role;

    let pass = password.unwrap_or(user.password.clone());
    user.password = hash_password(&pass);
    save_user(users);
}

fn main() {
    let cli = Args::parse();

    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add {
            name,
            password,
            admin,
        }) => add_user(name, password, admin.unwrap_or(false)),
        Some(Commands::Delete { username }) => delete_user(username),
        Some(Commands::Update {
            name,
            admin,
            password,
        }) => update_user(name, password, admin),
        None => println!("Run with --help"),
    }
}
