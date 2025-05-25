use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Kudos {
    pub id: u64,
    pub giver: String,
    pub receiver: String,
    pub message: String,
    pub timestamp: String,
    pub badge: String,
    
}  


#[component]
pub fn KudosCard(kudo: Kudos) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow p-6 hover:shadow-md transition-shadow",
            div { class: "flex items-center mb-4",
                span { class: "text-2xl mr-3", "{kudo.badge}" }
                div {
                    h3 { class: "font-medium text-gray-900", "{kudo.giver}" }
                    p { class: "text-sm text-gray-500", "recognized" }
                }
            }
            div { class: "pl-11",
                h4 { class: "text-lg font-medium text-blue-600 mb-1", "{kudo.receiver}" }
                p { class: "text-gray-600 mb-3", "{kudo.message}" }
                p { class: "text-sm text-gray-400", "{kudo.timestamp}" }
            }
        }
    }
}