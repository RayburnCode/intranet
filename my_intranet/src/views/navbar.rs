use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar(children: Element) -> Element {
    // State for mobile menu and user dropdown
    let mut mobile_menu_open = use_signal(|| false);
    let mut user_dropdown_open = use_signal(|| false);
    let is_admin = use_signal(|| false); // Replace with real auth check

    rsx! {
        nav { id: "navbar", class: "w-full bg-gray-800 text-white shadow-lg",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex items-center justify-between h-16",
                    // Left side - Logo and mobile menu button
                    div { class: "flex items-center",
                        // Mobile menu button (hidden on desktop)
                        button {
                            class: "md:hidden inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none",
                            onclick: move |_| mobile_menu_open.toggle(),
                            aria_label: "Toggle menu",
                            svg {
                                class: "h-6 w-6",
                                stroke: "currentColor",
                                fill: "none",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: match mobile_menu_open() {
                                        true => "M6 18L18 6M6 6l12 12",
                                        false => "M4 6h16M4 12h16M4 18h16",
                                    },
                                }
                            }
                        }
                        // Logo/Brand
                        Link {
                            to: Route::Home {},
                            class: "flex-shrink-0 flex items-center",
                            div { class: "flex items-center",
                                // Replace with your logo component or img tag
                                div { class: "h-8 w-8 bg-blue-500 rounded-full flex items-center justify-center text-white font-bold",
                                    "I"
                                }
                                span { class: "ml-2 text-xl font-semibold", "Intranet" }
                            }
                        }
                    }

                    // Desktop Navigation (hidden on mobile)
                    div { class: "hidden md:block",
                        div { class: "ml-10 flex items-baseline space-x-4",
                            NavLink { to: Route::Home {}, label: "Home" }
                            NavLink { to: Route::Directory {}, label: "Directory" }
                            NavLink { to: Route::Resources {}, label: "Resources" }
                            NavLink { to: Route::Tools {}, label: "Tools" }
                            NavLink {
                                to: Route::Departments {},
                                label: "Departments",
                            }
                            NavLink { to: Route::Social {}, label: "Social" }
                            // Admin link (conditional)
                            if is_admin() {
                                NavLink {
                                    to: Route::Admin {},
                                    label: "Admin",
                                    class: "text-red-300 hover:text-red-200",
                                }
                            }
                        }
                    }

                    // Right side - User profile
                    div { class: "ml-4 flex items-center md:ml-6",
                        // User dropdown
                        div { class: "relative",
                            button {
                                class: "max-w-xs flex items-center text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white",
                                onclick: move |_| user_dropdown_open.toggle(),
                                aria_label: "User menu",
                                span { class: "sr-only", "Open user menu" }
                                // User avatar - replace with your user image
                                div { class: "h-8 w-8 bg-gray-600 rounded-full flex items-center justify-center text-white",
                                    "U"
                                }
                            }
                            // Dropdown menu (conditionally shown)
                            if user_dropdown_open() {
                                div {
                                    class: "origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-white ring-1 ring-black ring-opacity-5 z-50",
                                    role: "menu",
                                    Link {
                                        to: Route::UserProfile {},
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        "Your Profile"
                                    }
                                    Link {
                                        to: Route::UserSettings {},
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        "Settings"
                                    }
                                    button {
                                        class: "block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                        onclick: move |_| {
                                            user_dropdown_open.set(false);
                                        },
                                        "Sign out"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Mobile menu (conditionally shown)
            if mobile_menu_open() {
                div { class: "md:hidden",
                    div { class: "px-2 pt-2 pb-3 space-y-1 sm:px-3",
                        MobileNavLink { to: Route::Home {}, label: "Home" }
                        MobileNavLink { to: Route::Directory {}, label: "Directory" }
                        MobileNavLink { to: Route::Resources {}, label: "Resources" }
                        MobileNavLink { to: Route::Tools {}, label: "Tools" }
                        MobileNavLink { to: Route::Departments {}, label: "Departments" }
                        MobileNavLink { to: Route::Social {}, label: "Social" }
                        if is_admin() {
                            MobileNavLink {
                                to: Route::Admin {},
                                label: "Admin",
                                class: "text-red-300 hover:text-red-200",
                            }
                        }
                    }
                }
            }
        }
        // Main content
        {children}
    }
}

// Reusable desktop nav link component
#[component]
fn NavLink(to: Route, label: &'static str, class: Option<&'static str>) -> Element {
    rsx! {
        Link {
            to,
            class: "px-3 py-2 rounded-md text-sm font-medium hover:bg-gray-700 hover:text-white {class.unwrap_or(\"\")}",
            active_class: "bg-gray-900 text-white",
            "{label}"
        }
    }
}

// Reusable mobile nav link component
#[component]
fn MobileNavLink(to: Route, label: &'static str, class: Option<&'static str>) -> Element {
    rsx! {
        Link {
            to,
            class: "block px-3 py-2 rounded-md text-base font-medium hover:bg-gray-700 hover:text-white {class.unwrap_or(\"\")}",
            active_class: "bg-gray-900 text-white",
            "{label}"
        }
    }}