//! Server funcs represent endpoints exposed by the server which can be accessed by the frontend.
//! You can call it like a regular function, but internally it sends a web request to the backend server.

use dioxus::prelude::*;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
