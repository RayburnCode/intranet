use dioxus::prelude::*;
use log;

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
                        active_tab: active_tab,
                        icon: "💬"
                    }
                    SocialTab {
                        label: "Kudos",
                        tab_id: "kudos",
                        active_tab: active_tab,
                        icon: "🏆"
                    }
                    SocialTab {
                        label: "Events",
                        tab_id: "events",
                        active_tab: active_tab,
                        icon: "📅"
                    }
                }
            }
            
            // Tab Content
            match &*active_tab() {
                "forums" => {
                    rsx! {
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
                                        d: "M12 4v16m8-8H4"
                                    }
                                }
                                span { class: "text-gray-600", "Start a new discussion" }
                            }
                            
                            // Posts List
                            for post in posts.iter() (
                                SocialPostCard { post: post.clone() }
                            )
                        }
                    }
                },
                "kudos" => {
                    rsx! {
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                            for kudo in kudos.iter() (
                                KudosCard { kudo: kudo.clone() }
                            )
                        }
                    }
                },
                "events" => {
                    rsx! {
                        div { class: "bg-white border rounded-lg p-6",
                            h2 { class: "text-xl font-semibold mb-4", "Upcoming Events" }
                            p { class: "text-gray-500 text-center py-8", "No upcoming events. Check back later!" }
                        }
                    }
                },
                _ => {
                    rsx! { div { "Invalid tab" } }
                }
            }
        }
    }
}

#[component]
pub fn SocialTab(label: &'static str, tab_id: &'static str, active_tab: Signal<String>, icon: &'static str) -> Element {
    rsx! {
        button {
            class: if active_tab() == tab_id {
                "whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center border-blue-500 text-blue-600"
            } else {
                "whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
            },
            onclick: move |_| active_tab.set(tab_id.to_string()),
            span { class: "mr-2 text-lg", "{icon}" }
            "{label}"
        }
    }
}

#[component]
pub fn SocialPostCard(post: SocialPost) -> Element {
    let likes = use_signal(|| post.likes);
    let is_liked = use_signal(|| post.is_liked);

    rsx! {
        div { class: "bg-white rounded-lg shadow p-6",
            div { class: "flex items-start",
                img { 
                    class: "w-10 h-10 rounded-full mr-4",
                    src: post.avatar,
                    alt: post.author
                }
                div { class: "flex-1",
                    div { class: "flex items-center mb-1",
                        h3 { class: "font-medium text-gray-900", "{post.author}" }
                        span { class: "mx-2 text-gray-400", "•" }
                        span { class: "text-sm text-gray-500", "{post.role}" }
                    }
                    p { class: "text-gray-600 mb-3", "{post.content}" }
                    div { class: "flex items-center text-sm text-gray-500",
                        span { "{post.timestamp}" }
                        span { class: "mx-2", "•" }
                        span { "{format!(\"{} comments\", post.comments)}" }
                    }
                }
            }
            div { class: "mt-4 flex items-center",
                button { 
                    class: if is_liked() { "flex items-center text-sm text-blue-500" } else { "flex items-center text-sm text-gray-500" },
                    onclick: move |_| {
                        is_liked.toggle();
                        likes.set(if is_liked() { post.likes + 1 } else { post.likes });
                    },
                    svg { 
                        class: "w-4 h-4 mr-1",
                        fill: if is_liked() { "currentColor" } else { "none" },
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                        }
                    }
                    span { likes() }
                }
                button { 
                    class: "ml-6 flex items-center text-sm text-gray-500",
                    svg { 
                        class: "w-4 h-4 mr-1",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                        }
                    }
                    span { "Comment" }
                }
            }
        }
    }
}

#[component]
fn KudosCard(kudo: Kudos) -> Element {
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

#[derive(Clone, PartialEq)]
struct SocialPost {
    id: u64,
    author: String,
    role: String,
    avatar: String,
    content: String,
    timestamp: String,
    likes: u32,
    comments: u32,
    is_liked: bool,
}

#[derive(Clone, PartialEq)]
struct Kudos {
    id: u64,
    giver: String,
    receiver: String,
    message: String,
    timestamp: String,
    badge: String,
}
