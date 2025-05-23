use crate::Route;
use dioxus::prelude::*;
// use crate::components::button::{Button, ButtonScheme, ButtonSize};
// const LOGO: Asset = asset!("/assets/logo.png");

#[component]
pub fn Navbar(children: Element) -> Element {
    // let handle_click = move |_| {
    //     println!("Button clicked!");
    // };

    rsx! {
        nav { id: "navbar", class: "w-full  text-white shadow-md",
            div { class: "px-8 py-2 mx-auto flex items-center justify-between",

                // Left side: Logo
                // a { href: "https://rayburnlp.com",
                //     img { class: "h-15", src: LOGO, alt: "Card header image" }
                // }


                // Right side: Links and Button
                div { class: "flex gap-6 items-center text-sm text-gray-900",
                    Link {
                        to: Route::Blog { id: 1 },
                        class: "hover:text-blue-400 transition",
                        {}
                        "Blog"
                    }
                    Link {
                        to: Route::Home {},
                        class: "hover:text-blue-200 transition",
                        "Home"
                    }
                    Link {
                        to: Route::Directory {},
                        class: "hover:text-blue-200 transition",
                        "Directory"
                    }
                    Link {
                        to: Route::Resources {},
                        class: "hover:text-blue-200 transition",
                        "Resources"
                    }
                    Link {
                        to: Route::Tools {},
                        class: "hover:text-blue-200 transition",
                        "Tools"
                    }
                    Link {
                        to: Route::Departments {},
                        class: "hover:text-blue-200 transition",
                        "Departments"
                    }

                    // Will need to change this
                    Link {
                        to: Route::Departments {},
                        class: "hover:text-blue-200 transition",
                        "Social"
                    }
                                // Conditional Admin Link (only shown to admins)
                // You'll need to implement your auth logic
                // {is_admin.then(|| rsx! {
                //     Link {
                //         to: Route::Admin {},
                //         class: "hover:text-blue-200 transition text-red-300",
                //         "Admin"
                //     }
                // })}

                // User Profile Dropdown would go here
                }
            }
        }
    }
    }
