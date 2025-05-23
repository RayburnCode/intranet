use dioxus::prelude::*;


#[component]
pub fn TicketFilterButton(label: &'static str, status: &'static str, current_status: Signal<String>) -> Element {
    let class = if current_status() == status {
        "px-4 py-2 rounded-full text-sm font-medium bg-blue-100 text-blue-800"
    } else {
        "px-4 py-2 rounded-full text-sm font-medium bg-gray-100 text-gray-800 hover:bg-gray-200"
    };

    rsx! {
        button {
            class: "{class}",
            onclick: move |_| current_status.set(status.to_string()),
            "{label}"
        }
    }
}

#[component]
pub fn TicketCard(
    id: u64,
    title: String,
    department: String,
    status: String,
    priority: String,
    assigned_to: String,
    created: String,
) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow p-5 hover:shadow-md transition-shadow",
            div { class: "flex justify-between items-start mb-2",
                h3 { class: "font-medium text-gray-900", "{title}" }
                {
                    let status_class = match status.as_str() {
                        "open" => "bg-yellow-100 text-yellow-800",
                        "closed" => "bg-green-100 text-green-800",
                        _ => "bg-gray-100 text-gray-800",
                    };
                    rsx! {
                        span { class: "text-xs px-2 py-1 rounded capitalize {status_class}", "{status}" }
                    }
                }
            }
            div { class: "flex flex-wrap gap-2 text-sm text-gray-600 mb-3",
                span { class: "flex items-center",
                    svg {
                        class: "w-3 h-3 mr-1",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4",
                        }
                    }
                    "{department}"
                }
                span { class: "flex items-center",
                    svg {
                        class: "w-3 h-3 mr-1",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                    }
                    "{created}"
                }
                span { class: "flex items-center",
                    svg {
                        class: "w-3 h-3 mr-1",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z",
                        }
                    }
                    "{assigned_to}"
                }
            }
            div { class: "flex justify-between items-center",
                Link { to: format!("/tools/tickets/{}", id),
                    {
                        let priority_class = match priority.as_str() {
                            "high" => "bg-red-100 text-red-800",
                            "medium" => "bg-orange-100 text-orange-800",
                            _ => "bg-gray-100 text-gray-800",
                        };
                        rsx! {
                            span { class: "text-xs px-2 py-1 rounded {priority_class}", "{priority} priority" }
                        }
                    }
                }
            }
        }
    }
}

