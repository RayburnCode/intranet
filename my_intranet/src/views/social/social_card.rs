use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SocialPost {
   pub id: u64,
   pub  author: String,
    pub role: String,
    pub avatar: String,
    pub content: String,
    pub timestamp: String,
    pub likes: u32,
    pub comments: u32,
    pub is_liked: bool,
}

#[component]
pub fn SocialPostCard(post: SocialPost) -> Element {
    let mut likes = use_signal(|| post.likes);
    let mut is_liked = use_signal(|| post.is_liked);

    rsx! {
        div { class: "bg-white rounded-lg shadow p-6",
            div { class: "flex items-start",
                img { class: "w-10 h-10 rounded-full mr-4", src: post.avatar }
                div { class: "flex-1",
                    div { class: "flex items-center mb-1",
                        h3 { class: "font-medium text-gray-900", {post.author} }
                        span { class: "mx-2 text-gray-400", "•" }
                        span { class: "text-sm text-gray-500", {post.role} }
                    }
                    p { class: "text-gray-600 mb-3", {post.content} }
                    div { class: "flex items-center text-sm text-gray-500",
                        span { {post.timestamp} }
                        span { class: "mx-2", "•" }
                        span { "{post.comments} comments" }
                    }
                }
            }
            div { class: "mt-4 flex items-center",
                button {
                    class: if *is_liked.read() { "flex items-center text-sm text-blue-500" } else { "flex items-center text-sm text-gray-500" },
                    onclick: move |_| {
                        let currently_liked = *is_liked.read();
                        let new_liked = !currently_liked;
                        is_liked.set(new_liked);
                        likes.set(if new_liked { post.likes + 1 } else { post.likes });
                    },
                    svg {
                        class: "w-4 h-4 mr-1",
                        fill: if *is_liked.read() { "currentColor" } else { "none" },
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z",
                        }
                    }
                    span { "{likes.read()}" }
                }
                button { class: "ml-6 flex items-center text-sm text-gray-500",
                    svg {
                        class: "w-4 h-4 mr-1",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z",
                        }
                    }
                    span { "Comment" }
                }
            }
        }
    }
}



   