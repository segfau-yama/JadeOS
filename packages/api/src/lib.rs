//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use std::process::Command;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Command::new("echo")
        .arg(&input)
        .spawn()
        .expect("error");
    Ok(input)
}
