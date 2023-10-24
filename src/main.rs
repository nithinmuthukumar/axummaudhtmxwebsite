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
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

mod page;
mod strings;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("src/static");

    let app = Router::new()
        .nest_service("/assets", serve_dir)
        .route("/", get(index))
        .route("/about", get(about_page))
        .route("/navbar", get(navbar))
        .fallback(not_found);

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
    let title = "Nithin Muthukumar";
    let desc = "Personal Website";
    let lang = "en";
    // TODO: Add your site or application content here.
    let content = html! {
        div hx-trigger="load" hx-get="/navbar?active=home"{}
        h1 {"Nithin Muthukumar"}

    };
    page::page(&host, title, desc, lang, content)
}
#[derive(Deserialize, Debug)]
pub struct NavParams {
    active: String,
}
async fn navbar(Query(params): Query<NavParams>) -> Markup {
    let pages = [("/", "Home"), ("/about", "About"), ("/resume", "Resume")];

    html! {
        (DOCTYPE)
        html{
            body {
                nav {
                    @for page in pages{
                        @if page.1.to_lowercase()==params.active{
                            a #active href=(page.0) {(page.1)}
                        }
                        @else{
                            a href=(page.0) {(page.1)}
                        }
                    }
                }
            }


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
