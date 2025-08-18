# Type Confusion Vulnerability Demonstration in Rust

This project demonstrates how a logic flaw similar to "Type Confusion" can be introduced and mitigated in a Rust actix-web application. While Rust's static typing prevents classic type confusion, vulnerabilities can arise from making incorrect assumptions after deserializing flexible data formats like JSON.

Lesson Summary
Vulnerability: The core issue stems from deserializing a JSON field into a generic serde_json::Value. The application logic then fails to handle cases where the value is not the expected type (e.g., receiving a JSON array when a string was expected). This allows validation checks to be bypassed, leading to a Cross-Site Scripting (XSS) vulnerability.

Mitigation: The fix is to enforce strict typing during deserialization. By defining the struct field as a String instead of serde_json::Value, we leverage Rust's type system and the serde framework to automatically reject any request where the data type is incorrect. This stops the malicious request before it reaches our application logic.

Project Structure
The application has two primary endpoints:

/vulnerable/signup: This endpoint accepts a JSON payload where the username can be any JSON type. It contains a flawed validation check that can be bypassed.
/secure/signup: This endpoint requires the username to be a JSON string. Any other type will result in an immediate 400 Bad Request error.

Setup and Running the Application
Prerequisites
Rust and Cargo installed (https://www.rust-lang.org/tools/install)

1. Create a New Rust Project
cargo new rust_type_confusion_demo
cd rust_type_confusion_demo

2. Add Dependencies
Add the following lines to your Cargo.toml file:

[dependencies]
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

3.Add the Code
Replace the contents of src/main.rs with the code from the main.rs file provided in this lesson.

4.Run the Application
cargo run

The server will start and listen on http://127.0.0.1:8080.

How to Test the Vulnerability

1. Demonstrate the Vulnerable Endpoint
We will send a POST request to the /vulnerable/signup endpoint with a JSON payload where the username is an array containing an XSS payload.

Attack Payload:

```bash
curl -X POST http://127.0.0.1:8080/vulnerable/signup \
-H "Content-Type: application/json" \
-d '{
    "username": ["<script>alert(\"XSS by Snyk Learn\")</script>"],
    "password": "password123"
}'

# Expected Vulnerable Output:
# The server will respond with 200 OK and the following HTML body. The validation was bypassed because the is_vulnerable_username_valid function doesn't correctly handle arrays.
# <h1>Thank you <script>alert("XSS by Snyk Learn")</script>, your account has been registered!</h1>
# If you were to render this in a browser, the JavaScript alert would execute.
```

2.Demonstrate the Secure Endpoint
Now, we will send the exact same malicious payload to the /secure/signup endpoint.

Secure Test:

```bash
curl -X POST http://127.0.0.1:8080/secure/signup \
-H "Content-Type: application/json" \
-d '{
    "username": ["<script>alert(\"XSS by Snyk Learn\")</script>"],
    "password": "password123"
}'

# Expected Secure Output:

# The server will immediately reject the request with a 400 Bad Request error. actix-web and serde prevent the request from even reaching our handler because the username field (an array) cannot be deserialized into the expected String type.

# The response body will be an error message similar to:
# Json deserialize error: invalid type: sequence, expected a string

# This demonstrates that by enforcing strict types at the deserialization boundary, the vulnerability is completely mitigated.
```
