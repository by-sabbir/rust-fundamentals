# rust-fundamentals - Authentication Project

This project is a simple authentication system implemented in Rust using the Cargo package manager. The system allows for user login and validation based on username and password.

## Features

- User registration and login functionality
- Validation of usernames and passwords
- Support for multiple user roles (Admin, User)

## Usage

```bash
cargo run
```

## Hot takes

- Options are for matching 2 possible outcomes
  - where one outcome can be matched with `Some`
  - `None` is like else statement
- `structs` can be used with `enum` vice versa
- `impl` binds methods with `enums` and `structs`
- `predicate` means function/closure
- `iter` passes by reference, the original items stays the same.
  - `into_iter` moves the value. it destroys the original items

## Use of `match` expression

The `match` expression in Rust is a powerful tool for handling different scenarios or outcomes of an operation. It's often used to provide more explicit and readable code when dealing with complex logic. The syntax for `match` is as follows: `match <expression> { |pattern| => <code>, ... }`.

In the context of this project, the `login` function uses a `match` expression to determine the outcome of a login attempt based on the provided username and password. It's a great example of how `match` can be used to simplify conditional logic and make code more maintainable.

For instance, in the `login` function, the `match` expression checks if the provided username and password match specific hardcoded values (e.g., "admin" and "password") or not. If they do, it returns a successful login result; otherwise, it returns an error message.

**Options and Pattern Matching in Rust**

In Rust, the `match` expression is used to handle different scenarios or outcomes of an operation. It's often used to provide more explicit and readable code when dealing with complex logic.

For example, in the `login` function of our authentication system, we can use a `match` expression to determine the outcome of a login attempt based on the provided username and password:

```rust
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    match (username, password) {
        ("admin", "password") => Some(LoginAction::Granted(LoginRole::Admin)),
        ("bob", "password") => Some(LoginAction::Granted(LoginRole::User)),
        (_, _) => None,
    }
}
```

In this example, we're using a tuple pattern to match against the username and password. The `_` wildcard is used to catch any other values that don't match our expected patterns.

This `match` expression is equivalent to writing:

```rust
if username == "admin" && password == "password" {
    return Some(LoginAction::Granted(LoginRole::Admin));
} else if username == "bob" && password == "password" {
    return Some(LoginAction::Granted(LoginRole::User));
} else {
    return None;
}
```

But the `match` expression is more concise and easier to read, especially when dealing with multiple branches or complex logic.

By using `match`, we can write more expressive and maintainable code that's easy to understand and modify.
