use dioxus::prelude::*;

pub mod tickets;
pub mod room_card;

use tickets::{TicketCard, TicketFilterButton};
use room_card::RoomCard;

#[component]
pub fn Tools() -> Element {
    let active_tab = use_signal(|| "tickets".to_string());
    let ticket_status = use_signal(|| "open".to_string()); // "open", "closed", "all"

    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            h1 { class: "text-3xl font-bold text-gray-800 mb-6", "Company Tools" }
            // Tab Navigation
            div { class: "border-b border-gray-200 mb-8",
                nav { class: "flex space-x-8",
                    ToolsTab {
                        label: "Tickets",
                        tab_id: "tickets",
                        active_tab,
                        icon: "🎫",
                    }
                    ToolsTab {
                        label: "Calendar",
                        tab_id: "calendar",
                        active_tab,
                        icon: "📅",
                    }
                    ToolsTab {
                        label: "Room Booking",
                        tab_id: "rooms",
                        active_tab,
                        icon: "🚪",
                    }
                }
            }
            // Tab Content
            match active_tab().as_str() {
                "tickets" => rsx! {
                    div {
                        // Ticket Filters
                        div { class: "flex flex-wrap gap-3 mb-6",
                            TicketFilterButton {
                                label: "Open Tickets",
                                status: "open",
                                current_status: ticket_status,
                            }
                            TicketFilterButton {
                                label: "Closed Tickets",
                                status: "closed",
                                current_status: ticket_status,
                            }
                            TicketFilterButton {
                                label: "All Tickets",
                                status: "all",
                                current_status: ticket_status,
                            }
                        }
                        // New Ticket Button
                        button {
                            class: "mb-6 flex items-center justify-center w-full p-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors",
                            onclick: move |_| log::info!("Create new ticket"),
                            svg {
                                class: "w-5 h-5 mr-2",
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
                            span { "Create New Ticket" }
                        }
                        // Tickets List
                        div { class: "space-y-4",
                            match ticket_status().as_str() {
                                "open" => rsx! {
                                    TicketCard {
                                        id: 1042,
                                        title: "VPN access not working".to_string(),
                                        department: "IT".to_string(),
                                        status: "open".to_string(),
                                        priority: "high".to_string(),
                                        assigned_to: "Tech Support Team".to_string(),
                                        created: "2 hours ago".to_string(),
                                    }
                                    TicketCard {
                                        id: 1041,
                                        title: "New monitor request".to_string(),
                                        department: "Facilities".to_string(),
                                        status: "open".to_string(),
                                        priority: "medium".to_string(),
                                        assigned_to: "Inventory Team".to_string(),
                                        created: "1 day ago".to_string(),
                                    }
                                },
                                "closed" => rsx! {
                                    TicketCard {
                                        id: 1040,
                                        title: "Printer jam in 3rd floor".to_string(),
                                        department: "Facilities".to_string(),
                                        status: "closed".to_string(),
                                        priority: "medium".to_string(),
                                        assigned_to: "Maintenance Team".to_string(),
                                        created: "3 days ago".to_string(),
                                    }
                                },
                                _ => rsx! {
                                    // Show all tickets
                                    TicketCard {
                                        id: 1042,
                                        title: "VPN access not working".to_string(),
                                        department: "IT".to_string(),
                                        status: "open".to_string(),
                                        priority: "high".to_string(),
                                        assigned_to: "Tech Support Team".to_string(),
                                        created: "2 hours ago".to_string(),
                                    }
                                    TicketCard {
                                        id: 1041,
                                        title: "New monitor request".to_string(),
                                        department: "Facilities".to_string(),
                                        status: "open".to_string(),
                                        priority: "medium".to_string(),
                                        assigned_to: "Inventory Team".to_string(),
                                        created: "1 day ago".to_string(),
                                    }
                                    TicketCard {
                                        id: 1040,
                                        title: "Printer jam in 3rd floor".to_string(),
                                        department: "Facilities".to_string(),
                                        status: "closed".to_string(),
                                        priority: "medium".to_string(),
                                        assigned_to: "Maintenance Team".to_string(),
                                        created: "3 days ago".to_string(),
                                    }
                                },
                            }
                        }
                    }
                },
                "calendar" => rsx! {
                    div { class: "bg-white border rounded-lg p-6",
                        h2 { class: "text-xl font-semibold mb-4", "Company Calendar" }
                        div { class: "grid grid-cols-7 gap-1 mb-4",
                            // Calendar Header
                            {
                                ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
                                    .iter()
                                    .map(|day| {
                                        rsx! {
                                            div { class: "text-center font-medium text-gray-500 py-2", "{day}" }
                                        }
                                    })
                            }
                            // Calendar Days (simplified)
                            {
                                (1..=31)
                                    .map(|day| {
                                        let highlight_class = if day == 15 {
                                            "bg-blue-50 border-blue-200"
                                        } else {
                                            ""
                                        };
                                        rsx! {
                                            div { class: "border p-2 h-24 overflow-y-auto {highlight_class}",
                                                div { class: "text-right text-sm", "{day}" }
                                                {
                                                    if day == 15 {
                                                        rsx! {
                                                            div { class: "text-xs mt-1 p-1 bg-blue-100 text-blue-800 rounded truncate", "All-Hands Meeting" }
                                                        }
                                                    } else {
                                                        rsx! {}
                                                    }
                                                }
                                            }
                                        }
                                    })
                            }
                        }
                    }
                },
                "rooms" => rsx! {
                    div {
                        h2 { class: "text-xl font-semibold mb-4", "Book a Meeting Room" }
                        // Room Grid
                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                            RoomCard {
                                name: "Conference Room A".to_string(),
                                capacity: "8 people".to_string(),
                                floor: "3rd Floor".to_string(),
                                status: "available".to_string(),
                                next_booking: "Available until 3pm".to_string(),
                            }
                            RoomCard {
                                name: "War Room".to_string(),
                                capacity: "12 people".to_string(),
                                floor: "2nd Floor".to_string(),
                                status: "booked".to_string(),
                                next_booking: "Booked until 1pm".to_string(),
                            }
                            RoomCard {
                                name: "Quiet Room".to_string(),
                                capacity: "4 people".to_string(),
                                floor: "1st Floor".to_string(),
                                status: "available".to_string(),
                                next_booking: "Available all day".to_string(),
                            }
                        }
                        // Booking Form (would be a modal in real implementation)
                        div { class: "mt-8 bg-white border rounded-lg p-6",
                            h3 { class: "text-lg font-medium mb-4", "New Booking" }
                            div { class: "space-y-4",
                                div {
                                    label { class: "block text-sm font-medium text-gray-700 mb-1", "Room" }
                                    select { class: "w-full p-2 border rounded",
                                        option { "Conference Room A" }
                                        option { "War Room" }
                                        option { "Quiet Room" }
                                    }
                                }
                                div { class: "grid grid-cols-2 gap-4",
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Date" }
                                        input { class: "w-full p-2 border rounded", r#type: "date" }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-gray-700 mb-1", "Time" }
                                        input { class: "w-full p-2 border rounded", r#type: "time" }
                                    }
                                }
                                button { class: "mt-4 w-full p-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                                    "Book Room"
                                }
                            }
                        }
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
fn ToolsTab(label: &'static str, tab_id: &'static str, active_tab: ReadSignal<String>, icon: &'static str) -> Element {
    rsx! {
fn ToolsTab(label: &'static str, tab_id: &'static str, active_tab: ReadSignal<String>, icon: &'static str) -> Element {
            class: {
                if active_tab.get() == tab_id {
                    "whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center border-blue-500 text-blue-600"
                } else {
                    "whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm flex items-center border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
                }
            },
            onclick: move |_| active_tab.set(tab_id.to_string()),
            span { class: "mr-2 text-lg", "{icon}" }
            "{label}"
        }
    }
}

