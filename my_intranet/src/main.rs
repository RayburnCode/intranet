
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{ AppLayout, Home, Blog};
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
   // #[layout(Navbar)]
   #[layout(AppLayout)]

        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
        #[route("/directory")]
        Directory {},
        #[route("/resources")]
        Resources {},
        #[route("/tools")]
        Tools {},
        #[route("/departments")]
        Departments {},
        #[route("/social")]
        Social {},
}


const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {

    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}