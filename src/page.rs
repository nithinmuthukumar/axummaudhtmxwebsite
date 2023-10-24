use crate::strings;
use maud::{html, Markup, DOCTYPE};

fn body(content: Markup) -> Markup {
    html! {
        body {
            (content)
            script src="assets/js/vendor/htmx.min.js" {}
            script src="assets/js/vendor/modernizr-3.11.2.min.js" {}
            script src="assets/js/plugins.js" {}
            script src="assets/js/main.js" {}
        }
    }
}

fn head(title: &str, desc: &str, url: &str) -> Markup {
    html! {
        head {
            meta charset=(strings::UTF8);
            title { (title) }
            meta name=(strings::DESCRIPTION) content=(desc);
            meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
            meta property="og:title" content=(title);
            meta property="og:type" content=(strings::WEBSITE);
            meta property="og:url" content=(url);
            meta property="og:image" content="";
            link rel="manifest" href="assets/site.webmanifest";
            link rel="apple-touch-icon" href="assets/icon.png";
            link rel="stylesheet" href="assets/css/normalize.css";
            link rel="stylesheet" href="assets/css/main.css";
            link rel="stylesheet" href="assets/css/style.css";

            meta name="theme-color" content="#fafafa";
        }
    }
}

pub(crate) fn page(content: Markup) -> Markup {
    let host = "nithinmuthukumar.com";
    let title = "Nithin Muthukumar";
    let desc = "Personal Website";
    let lang = "en";

    html! {
        (DOCTYPE)
        html class="no-js" lang=(lang) {
            (head(title, desc, host))
            (body(content))
        }
    }
}
