// main.rs

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// --- Vulnerable Implementation ---

/// Represents the structure for user signup in the vulnerable endpoint.
/// It uses `serde_json::Value` for the username, which allows any valid JSON type.
/// This flexibility is the source of our simulated "type confusion".
#[derive(Deserialize, Debug)]
struct VulnerableUser {
    username: Value,
    password: String,
}

/// A simple validation function that incorrectly assumes `username` is a string.
/// This is where the logic flaw lies. If `username` is not a string, the `as_str()`
/// method will return `None`, causing the validation to be bypassed.
fn is_vulnerable_username_valid(username: &Value) -> bool {
    if let Some(s) = username.as_str() {
        // If it's a string, check for forbidden characters.
        !s.contains('<') && !s.contains('>')
    } else {
        // If it's not a string (e.g., an array or object), the validation is bypassed.
        // The function incorrectly assumes the username is valid in this case.
        true
    }
}

/// The vulnerable signup handler.
/// It deserializes the request into `VulnerableUser`, which uses `serde_json::Value`.
/// The `is_vulnerable_username_valid` function can be bypassed if the username
/// is sent as a JSON array instead of a string.
async fn vulnerable_signup(user: web::Json<VulnerableUser>) -> impl Responder {
    println!("Vulnerable endpoint received: {:?}", user);

    if !is_vulnerable_username_valid(&user.username) {
        return HttpResponse::BadRequest().body("Username contains invalid characters.");
    }

    // The `user.username` Value is implicitly converted to a string here using the
    // default `Display` trait implementation for `Value`, which can be exploited.
    // For an array `["<script>alert(1)</script>"]`, this becomes `<script>alert(1)</script>`.
    let response_body = format!(
        "<h1>Thank you {}, your account has been registered!</h1>",
        user.username.to_string().replace("\"", "") // Remove quotes for cleaner HTML output
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body)
}

// --- Secure Implementation ---

/// Represents the structure for user signup in the secure endpoint.
/// The `username` field is explicitly typed as a `String`.
/// `serde` will fail to deserialize if the incoming JSON value for `username`
/// is not a string, preventing the type manipulation at the source.
#[derive(Deserialize, Debug)]
struct SecureUser {
    username: String,
    password: String,
}

/// This validation function now safely operates on a `&str` because the type
/// is guaranteed by the `SecureUser` struct.
fn is_secure_username_valid(username: &str) -> bool {
    !username.contains('<') && !username.contains('>')
}

/// The secure signup handler.
/// It uses `SecureUser`, which enforces that `username` must be a string.
/// Any attempt to send a different JSON type (like an array) will result in a
// * `400 Bad Request` from `actix-web` before our handler is even called.
async fn secure_signup(user: web::Json<SecureUser>) -> impl Responder {
    println!("Secure endpoint received: {:?}", user);

    if !is_secure_username_valid(&user.username) {
        return HttpResponse::BadRequest().body("Username contains invalid characters.");
    }

    let response_body = format!(
        "<h1>Thank you {}, your account has been registered!</h1>",
        user.username
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body)
}

/// Main function to set up and run the actix-web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            // Route for the vulnerable version
            .route("/vulnerable/signup", web::post().to(vulnerable_signup))
            // Route for the secure version
            .route("/secure/signup", web::post().to(secure_signup))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
