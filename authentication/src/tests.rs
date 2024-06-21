use super::*;

#[test]
fn test_login_admin() {
    let username = "admin";
    let password = "password";
    let result = login(username, password);
    match result {
        Some(LoginAction::Granted(role)) => assert_eq!(role, LoginRole::Admin),
        _ => panic!("Expected admin role"),
    }
}

#[test]
fn test_login_user() {
    let username = "bob";
    let password = "password";
    let result = login(username, password);
    match result {
        Some(LoginAction::Granted(role)) => assert_eq!(role, LoginRole::User),
        _ => panic!("Expected user role"),
    }
}

#[test]
fn test_login_denied() {
    let username = "bob";
    let password = "pasdsword";
    let result = login(username, password);
    match result {
        Some(LoginAction::Denied) => assert!(true),
        _ => panic!("Expected denied action"),
    }
}

#[test]
fn test_login_invalid_input() {
    let username = "";
    let password = "";
    let result = login(username, password);
    match result {
        None => assert!(true),
        _ => panic!("Expected none for invalid input"),
    }
}
