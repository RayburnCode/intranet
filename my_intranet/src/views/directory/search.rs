use dioxus::prelude::*;


#[component]
pub fn EmployeeSearch(filters: Vec<String>) -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut selected_filter = use_signal(|| filters.get(0).cloned().unwrap_or_default());

    rsx! {
        div { class: "bg-white rounded-lg shadow p-6 sticky top-6",
            h2 { class: "text-xl font-semibold text-gray-700 mb-4", "Find Colleagues" }
            // Search Bar
            div { class: "relative mb-4",
                input {
                    class: "w-full p-3 pl-10 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                    placeholder: "Search employees...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
                svg {
                    class: "absolute left-3 top-3.5 h-5 w-5 text-gray-400",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                    }
                }
            }
            // Filters
            div { class: "mb-4",
                label { class: "block text-sm font-medium text-gray-700 mb-2", "Filter by:" }
                div { class: "flex flex-wrap gap-2",
                    {
                        filters
                            .iter()
                            .map(|filter| {
                                let filter = filter.clone();
                                let is_selected = *selected_filter.read() == filter;
                                rsx! {
                                    button {
                                        class: if is_selected { "px-3 py-1 text-sm rounded-full bg-blue-100 text-blue-800 border border-blue-300" } else { "px-3 py-1 text-sm rounded-full bg-gray-100 text-gray-800 hover:bg-gray-200" },
                                        onclick: move |_| selected_filter.set(filter.clone()),
                                        "{filter}"
                                    }
                                }
                            })
                    }
                }
            }
            // Search Results (would be populated from API)
            div { class: "space-y-3",
                p { class: "text-center text-gray-500 py-8", "Start typing to search employees" }
                // Example result (would be in a loop)
                div { class: "hidden", // Remove hidden when implementing
                    div { class: "flex items-center p-2 hover:bg-gray-50 rounded",
                        img {
                            class: "w-10 h-10 rounded-full mr-3",
                            src: "/avatars/sample.png",
                            alt: "",
                        }
                        div {
                            p { class: "font-medium", "Sample Employee" }
                            p { class: "text-sm text-gray-500", "Sample Department" }
                        }
                    }
                }
            }
        }
    }
}