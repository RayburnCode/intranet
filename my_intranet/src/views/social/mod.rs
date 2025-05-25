use dioxus::prelude::*;
use log;

pub mod kudos;
pub mod social_card;
use kudos::{KudosCard, Kudos};
use social_card::{SocialPostCard,SocialPost};




#[component]
pub fn Social() -> Element {
    let active_tab = use_signal(|| "forums".to_string());
    let posts = vec![
        SocialPost {
            id: 1,
            author: "Alex Chen".to_string(),
            role: "Engineering Manager".to_string(),
            avatar: "/avatars/alex.png".to_string(),
            content: "Does anyone have experience with the new Rust WASM tools? We're evaluating for a client project.".to_string(),
            timestamp: "2 hours ago".to_string(),
            likes: 8,
            comments: 4,
            is_liked: false,
        },
        SocialPost {
            id: 2,
            author: "Jamie Smith".to_string(),
            role: "HR Specialist".to_string(),
            avatar: "/avatars/jamie.png".to_string(),
            content: "Reminder: Our quarterly wellness webinar is tomorrow at 2pm in the main conference room!".to_string(),
            timestamp: "1 day ago".to_string(),
            likes: 15,
            comments: 3,
            is_liked: true,
        },
    ];

    let kudos = vec![
        Kudos {
            id: 1,
            giver: "Taylor Wong".to_string(),
            receiver: "Morgan Lee".to_string(),
            message: "For helping debug the production issue at 2AM!".to_string(),
            timestamp: "3 days ago".to_string(),
            badge: "🏆 Hero".to_string(),
        },
        Kudos {
            id: 2,
            giver: "Casey Kim".to_string(),
            receiver: "Team Infrastructure".to_string(),
            message: "The seamless migration to the new servers was incredible!".to_string(),
            timestamp: "1 week ago".to_string(),
            badge: "👏 Teamwork".to_string(),
        },
    ];

    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            h1 { class: "text-3xl font-bold text-gray-800 mb-6", "Social Hub" }
            // Tab Navigation
            div { class: "border-b border-gray-200 mb-8",
                nav { class: "flex space-x-8",
                    SocialTab {
                        label: "Forums",
                        tab_id: "forums",
                        active_tab,
                        icon: "💬",
                    }
                    SocialTab {
                        label: "Kudos",
                        tab_id: "kudos",
                        active_tab,
                        icon: "🏆",
                    }
                    SocialTab {
                        label: "Events",
                        tab_id: "events",
                        active_tab,
                        icon: "📅",
                    }
                }
            }
            // Tab Content
            match active_tab().as_str() {
                "forums" => rsx! {
                    div { class: "space-y-6",
                        // New Post Button
                        button {
                            class: "flex items-center justify-center w-full p-4 border-2 border-dashed border-gray-300 rounded-lg hover:border-blue-500 transition-colors",
                            onclick: move |_| log::info!("Create new post"),
                            svg {
                                class: "w-5 h-5 mr-2 text-gray-500",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M12 4v16m8-8H4",
                                }
                            }
                            span { class: "text-gray-600", "Start a new discussion" }
                        }
                        // Posts List
                        for post in posts {
                            SocialPostCard { key: "{post.id}", post: post.clone() }
                        }
                    }
                },
                "kudos" => rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        for kudo in kudos {
                            KudosCard { key: "{kudo.id}", kudo: kudo.clone() }
                        }
                    }
                },
                "events" => rsx! {
                    div { class: "bg-white border rounded-lg p-6",
                        h2 { class: "text-xl font-semibold mb-4", "Upcoming Events" }
                        p { class: "text-gray-500 text-center py-8", "No upcoming events. Check back later!" }
                    }
                },
                _ => rsx! {
                    div { "Invalid tab" }
                },
            }
        }
    }
}

#[component]
pub fn SocialTab(
    label: &'static str,
    tab_id: &'static str,
    active_tab: Signal<String>,
    icon: &'static str,
) -> Element {
    let is_active = active_tab() == tab_id;
    
   rsx! {
    button {
        class: if is_active { "whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center border-blue-500 text-blue-600" } else { "whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300" },
        onclick: move |_| active_tab.set(tab_id.to_string()),
        span { class: "mr-2 text-lg", "{icon}" }
        "{label}"
    }
}
}