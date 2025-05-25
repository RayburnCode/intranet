
use dioxus::prelude::*;

#[component]
pub fn UserSettings() -> Element {
    rsx! {
        Link {
            to: "/settings",
            class: "group block p-6 border rounded-lg hover:border-blue-500 hover:shadow-md transition-all duration-200",
            div { class: "flex items-start",
                span { class: "text-3xl mr-4", "Settings" }
                div {
                    h3 { class: "text-xl font-semibold text-gray-800 group-hover:text-blue-600 mb-1",
                        "Settings"
                    }
                    p { class: "text-gray-600 text-sm", "Settings" }
                }
            }
        }
    }
}