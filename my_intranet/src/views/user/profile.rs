use dioxus::prelude::*;

#[component]
pub fn UserProfile() -> Element {
    rsx! {
        Link {
            to: "/profile",
            class: "group block p-6 border rounded-lg hover:border-blue-500 hover:shadow-md transition-all duration-200",
            div { class: "flex items-start",
                span { class: "text-3xl mr-4", "Profile" }
                div {
                    h3 { class: "text-xl font-semibold text-gray-800 group-hover:text-blue-600 mb-1",
                        "Profile"
                    }
                    p { class: "text-gray-600 text-sm", "profile" }
                }
            }
        }
    }
}