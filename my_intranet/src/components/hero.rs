use dioxus::prelude::*;

//mconst HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            h1 { "Company Intranet" }
            p { "Welcome to our internal portal" }
                // Add your intranet components here
        }
    }
}
