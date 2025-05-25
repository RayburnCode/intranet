use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Home() -> Element {
    // Sample data - replace with real data from your backend
    let announcements = vec![
        Announcement {
            id: 1,
            title: "Office Closure for Holidays".to_string(),
            content: "The office will be closed December 25-January 1 for the winter holidays.".to_string(),
            date: "December 15".to_string(),
            priority: "high".to_string(),
        },
        Announcement {
            id: 2,
            title: "New Benefits Portal".to_string(),
            content: "Our updated benefits portal is now live with enhanced features.".to_string(),
            date: "December 10".to_string(),
            priority: "normal".to_string(),
        },
    ];

    let quick_links = vec![
        QuickLink {
            title: "HR Portal".to_string(),
            description: "Access benefits and policies".to_string(),
            icon: "👥".to_string(),
            route: Route::Departments {},
        },
        QuickLink {
            title: "IT Tickets".to_string(),
            description: "Submit tech support requests".to_string(),
            icon: "💻".to_string(),
            route: Route::Tools {},
        },
        QuickLink {
            title: "Expense Reports".to_string(),
            description: "Submit and track expenses".to_string(),
            icon: "💰".to_string(),
            route: Route::Resources {},
        },
    ];

    let metrics = vec![
        Metric {
            title: "Open Tickets".to_string(),
            value: "24".to_string(),
            change: "+2".to_string(),
            trend: "up".to_string(),
        },
        Metric {
            title: "Active Projects".to_string(),
            value: "15".to_string(),
            change: "-3".to_string(),
            trend: "down".to_string(),
        },
        Metric {
            title: "New Hires".to_string(),
            value: "5".to_string(),
            change: "+1".to_string(),
            trend: "up".to_string(),
        },
    ];

   rsx! {
    div { class: "p-6 max-w-7xl mx-auto space-y-8",
        // Welcome Banner
        div { class: "bg-gradient-to-r from-blue-600 to-blue-800 rounded-lg p-6 text-white",
            h1 { class: "text-2xl font-bold mb-2", "Welcome to the Company Intranet" }
            p { "Today is {chrono::Local::now().format(\"%A, %B %e\")}" }
        }

        // Announcements Carousel
        div { class: "bg-white rounded-lg shadow",
            h2 { class: "p-4 text-xl font-semibold border-b", "Announcements" }
            div { class: "divide-y",
                for announcement in announcements.iter() {
                    AnnouncementCard { announcement: announcement.clone() }
                }
            }
                // Link {
        //     to: Route::Announcements {},
        //     class: "block p-4 text-center text-blue-600 hover:bg-blue-50 rounded-b-lg",
        //     "View All Announcements"
        // }
        }

        // Quick Links
        div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
            for link in quick_links.iter() {
                QuickLinkCard { link: link.clone() }
            }
        }

        // Metrics Dashboard
        div { class: "bg-white rounded-lg shadow p-6",
            h2 { class: "text-xl font-semibold mb-4", "Company Metrics" }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                for metric in metrics.iter() {
                    MetricCard { metric: metric.clone() }
                }
            }
        }

        // Recent Activity
        div { class: "bg-white rounded-lg shadow",
            h2 { class: "p-4 text-xl font-semibold border-b", "Recent Activity" }
            div { class: "p-4 text-gray-500 text-center", "Recent employee activity will appear here" }
        }
    }
}
}

// Components
#[component]
fn AnnouncementCard(announcement: Announcement) -> Element {
    rsx! {
        div { class: "p-4 hover:bg-gray-50 transition-colors",
            div { class: "flex justify-between items-start mb-1",
                h3 { class: "font-medium text-gray-900", {announcement.title.clone()} }
                span { class: "text-sm text-gray-500", {announcement.date.clone()} }
            }
            p { class: "text-gray-600 mb-2", {announcement.content.clone()} }
            if announcement.priority == "high" {
                div { class: "text-xs text-red-600 font-medium", "❗ High Priority" }
            }
        }
    }
}

#[component]
fn QuickLinkCard(link: QuickLink) -> Element {
    rsx! {
        Link {
            to: link.route.clone(),
            class: "bg-white rounded-lg shadow p-6 hover:shadow-md transition-shadow flex items-start",
            span { class: "text-3xl mr-4", {link.icon.clone()} }
            div {
                h3 { class: "text-lg font-medium text-gray-800 mb-1", {link.title.clone()} }
                p { class: "text-gray-600 text-sm", {link.description.clone()} }
            }
        }
    }
}

#[component]
fn MetricCard(metric: Metric) -> Element {
    rsx! {
        div { class: "border rounded-lg p-4",
            p { class: "text-gray-500 text-sm mb-1", {metric.title.clone()} }
            div { class: "flex items-baseline",
                span { class: "text-2xl font-bold mr-2", {metric.value.clone()} }
                span {
                    class: {
                        match metric.trend.as_str() {
                            "up" => "text-sm text-green-600",
                            _ => "text-sm text-red-600",
                        }
                    },
                    {metric.change.clone()}
                }
            }
        }
    }
}

// Data Structures
#[derive(Clone, PartialEq)]
struct Announcement {
    id: u64,
    title: String,
    content: String,
    date: String,
    priority: String,
}

#[derive(Clone, PartialEq)]
struct QuickLink {
    title: String,
    description: String,
    icon: String,
    route: Route,
}

#[derive(Clone, PartialEq)]
struct Metric {
    title: String,
    value: String,
    change: String,
    trend: String,
}