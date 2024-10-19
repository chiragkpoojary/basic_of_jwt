use actix_web::{get, post, web, App, HttpResponse, HttpServer};
mod jwt;
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[get("/signin")]
async fn signin() -> HttpResponse {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Sign In</title>
    </head>
    <body>
        <h1>Sign In</h1>
        <form action="/signin" method="post">
            <label for="username">Username:</label>
            <input type="text" id="username" name="username"><br><br>
            <label for="password">Password:</label>
            <input type="password" id="password" name="password"><br><br>
            <input type="submit" value="Sign In">
        </form>
    </body>
    </html>
    "#;

    HttpResponse::Ok().content_type("text/html").body(html)
}

#[post("/signin")]
async fn login(login_request: web::Form<LoginRequest>) -> HttpResponse {
    if login_request.username == "user" && login_request.password == "password" {
        let token = jwt::create_jwt(&login_request.username);
        let response_html = format!(
            r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>JWT Token</title>
        </head>
        <body>
            <h1>Login Successful</h1>
            <p>Your JWT token is:</p>
            <pre>{}</pre>
        </body>
        </html>
        "#,
            token
        );
        HttpResponse::Ok()
            .content_type("text/html")
            .body(response_html)
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(signin)
            .service(login)
            .route("/", web::get().to(hello_world))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
