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
        div class ="container mx-auto py-8"{
            div class ="bg-white p-4 rounded-lg shadow-lg"{
                h2 class="text-2xl font-semibold"{
                    "Hey There!"
                }

                p class="mt-4"{
                    "I'm currently a 4th year Computer Science student at University of Windsor."
                }
                p class="mt-4"{
                    "I'm currently a 4th year Computer Science student at University of Windsor."
                }

            }

        }

    };
    page::page(content)
}

async fn index() -> Markup {
    let content = html! {
        div hx-trigger="load" hx-get="/navbar?active=home"{}
        div class="min-h-screen flex items-center justify-center flex-col space-y-10"{
            div class=""{
                h1 class="text-center text-red-800 text-5xl md:text-7xl lg:text-9xl font-bold "{"Nithin Muthukumar"}
            }
            div class="mx-auto flex justify-between w-1/4"{

                a href="https://github.com/nithinmuthukumar"{

                    img class="w-20 transition-transform transform scale-100 hover:scale-110" src="assets/img/github_icon.png";
                }
                a href="https://devpost.com/nithinmuthukumar"{

                img class="w-24 transition-transform transform scale-100 hover:scale-110" src="assets/img/devpost_icon.png";
                }
                a href="https://nithinmuthukumar.itch.io"{

                    img class="w-20 transition-transform transform scale-100 hover:scale-110 " src="assets/img/itch_icon.png";
                }
                a href="https://www.linkedin.com/in/nithin-muthukumar-681219162/"{

                img class="w-20 transition-transform transform scale-100 hover:scale-110" src="assets/img/linkedin_icon.png";
                }

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
        nav class="bg-gray-100 p-4"{
            ul class="flex space-x-4"{
            @for page in pages{
                @if page.1.to_lowercase()==params.active{
                    li{
                    a class="text-white bg-red-500 hover:bg-red-700 px-4 py-2 rounded font-bold disabled" href=(page.0) {(page.1)}


                    }
                }
                @else{
                    li{a class="text-red-500 hover:text-red-700 hover:underline px-4 py-2 rounded" href=(page.0) {(page.1)}
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
