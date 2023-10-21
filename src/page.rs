use crate::strings;
use maud::{html, Markup, DOCTYPE};

fn body(content: Markup) -> Markup {
    html! {
        body {
            (content)
            script src="js/vendor/htmx.min.js" {}
            script src="js/vendor/modernizr-3.11.2.min.js" {}
            script src="js/plugins.js" {}
            script src="js/main.js" {}
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
            link rel="manifest" href="site.webmanifest";
            link rel="apple-touch-icon" href="icon.png";
            link rel="stylesheet" href="css/normalize.css";
            link rel="stylesheet" href="css/main.css";
            meta name="theme-color" content="#fafafa";
        }
    }
}

pub(crate) fn page(host: &str, title: &str, desc: &str, lang: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html class="no-js" lang=(lang) {
            (head(title, desc, host))
            (body(content))
        }
    }
}
