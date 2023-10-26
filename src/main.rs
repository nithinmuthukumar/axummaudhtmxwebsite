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

async fn about_page() -> Markup {
    let content = html! {
        div hx-trigger="load" hx-get="/navbar?active=about"{}
    };
    page::page(content)
}

async fn index() -> Markup {
    let content = html! {
        div hx-trigger="load" hx-get="/navbar?active=home"{}
        div #home{
        div #intro{
            h1 class="text-red-800 font-bold"{"Nithin Muthukumar"}
        }
        div .links{
            img .icon src="assets/img/github_icon.png";
            img .icon src="assets/img/devpost_icon.png";
            img .icon src="assets/img/itch_icon.png";
            img .icon src="assets/img/linkedin_icon.png";


        }
        }






    };
    page::page(content)
}
#[derive(Deserialize, Debug)]
pub struct NavParams {
    active: String,
}
async fn navbar(Query(params): Query<NavParams>) -> Markup {
    let pages = [("/", "Home"), ("/about", "About"), ("/resume", "Resume")];

    html! {
        nav {
            ul{
            @for page in pages{
                @if page.1.to_lowercase()==params.active{
                    li{
                    a #active href=(page.0) {(page.1)}


                    }
                }
                @else{
                    li{a class="hover:underline" href=(page.0) {(page.1)}
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
