use dioxus::prelude::*;
use crate::views::{Blog};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // let handle_click = move |_| {
    //     println!("Button clicked!");
    // };
    rsx! {
        Blog { id: 1 }
    }
}