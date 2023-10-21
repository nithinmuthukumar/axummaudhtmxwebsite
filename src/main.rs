use axum::{
    extract::Query,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::{net::SocketAddr, path::Path};
use tower_http::services::{ServeDir, ServeFile};

mod page;
mod strings;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("src/static");

    let app = Router::new()
        .nest_service("/", serve_dir)
        .route("/home", get(index))
        .route("/hello", post(hello))
        .route("/about", get(about_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn about_page() -> String {
    "Hellow".to_string()
}

async fn index() -> Markup {
    let host = format!("{}", "Nithin");
    let title = "actix-maud-htmx-h5bp";
    let desc = "This is a template. There are many like it but this one is mine.";
    let lang = "en";
    // TODO: Add your site or application content here.
    let content = html! {
        #content {
            p { "Hello world! This is HTML5 Boilerplate." }
        }
        form hx-post="/hello" hx-target="#content" hx-swap="outerHTML" {
            div {
                label { "What's your name? " }
                input type="text" name="name" value="" {}
            }
            button { "Submit" }
        }
    };
    page::page(&host, title, desc, lang, content)
}
#[derive(Deserialize)]
struct UserInfo {
    name: String,
}

async fn hello(Form(user): Form<UserInfo>) -> Markup {
    html! {
        #content {
            p { "Hello " (user.name) "! This is HTMX." }
        }
    }
}
async fn not_found() -> Markup {
    html! {
        html lang="en" {
            head {
                meta charset=(strings::UTF8);
                title { (strings::NOT_FOUND_TITLE) }
                meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
                style { (strings::NOT_FOUND_STYLE) }
            }
            body {
                h1 { (strings::NOT_FOUND_TITLE) }
                p { (strings::NOT_FOUND_MESSAGE) }
            }
            (PreEscaped(strings::NOT_FOUND_COMMENT))
        }
    }
}
