use dioxus::prelude::*;



#[component]
pub fn RoomCard(
    name: String,
    capacity: String,
    floor: String,
    status: String,
    next_booking: String,
) -> Element {
    rsx! {
        div { class: "bg-white border rounded-lg p-5 hover:shadow-md transition-shadow",
            div { class: "flex justify-between items-start mb-2",
                h3 { class: "font-medium text-gray-900", "{name}" }
                span {
                    class: {
                        if status == "available" {
                            "text-xs px-2 py-1 rounded bg-green-100 text-green-800"
                        } else {
                            "text-xs px-2 py-1 rounded bg-red-100 text-red-800"
                        }
                    },
                    "{status}"
                }
            }
            div { class: "space-y-2 text-sm text-gray-600 mb-4",
                div { class: "flex items-center",
                    svg {
                        class: "w-4 h-4 mr-2 text-gray-400",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z",
                        }
                    }
                    "{capacity}"
                }
                div { class: "flex items-center",
                    svg {
                        class: "w-4 h-4 mr-2 text-gray-400",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
                        }
                    }
                    "{floor}"
                }
                div { class: "flex items-center",
                    svg {
                        class: "w-4 h-4 mr-2 text-gray-400",
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
                    button {
                        class: {
                            let base = "w-full p-2 bg-blue-600 text-white text-sm rounded hover:bg-blue-700";
                            if status != "available" {
                                format!("{base} opacity-50 cursor-not-allowed")
                            } else {
                                base.to_string()
                            }
                        },
                        disabled: status != "available",
                        "Book Now"
                    }
                }
            }
        }
    }}