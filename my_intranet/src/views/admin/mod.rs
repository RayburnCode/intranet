use dioxus::prelude::*;
use crate::Route;

pub mod setting_toggle;
use setting_toggle::SettingToggle;
pub mod user_edit;
pub use user_edit::AdminUserEdit;
pub mod user_create;
pub use user_create::AdminUserCreate;


#[component]
pub fn Admin() -> Element {
    // Sample admin data - replace with real data from your backend
    let users = vec![
        AdminUser {
            id: 1,
            name: "Alice Johnson".to_string(),
            email: "alice@company.com".to_string(),
            role: "Admin".to_string(),
            last_active: "2 hours ago".to_string(),
        },
        AdminUser {
            id: 2,
            name: "Bob Smith".to_string(),
            email: "bob@company.com".to_string(),
            role: "Manager".to_string(),
            last_active: "1 day ago".to_string(),
        },
        AdminUser {
            id: 3,
            name: "Charlie Brown".to_string(),
            email: "charlie@company.com".to_string(),
            role: "User".to_string(),
            last_active: "3 days ago".to_string(),
        },
    ];

    let system_stats = AdminStats {
        total_users: 42,
        active_today: 28,
        storage_used: "65%".to_string(),
        open_tickets: 15,
    };

    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            // Admin header
            div { class: "mb-8",
                h1 { class: "text-3xl font-bold text-gray-800", "Admin Dashboard" }
                p { class: "text-gray-600", "Manage system settings and users" }
            }

            // Quick stats cards
            div { class: "grid grid-cols-1 md:grid-cols-4 gap-4 mb-8",
                StatCard {
                    title: "Total Users",
                    value: system_stats.total_users.to_string(),
                    icon: "👥",
                }
                StatCard {
                    title: "Active Today",
                    value: system_stats.active_today.to_string(),
                    icon: "🟢",
                }
                StatCard {
                    title: "Storage Used",
                    value: system_stats.storage_used.clone(),
                    icon: "💾",
                }
                StatCard {
                    title: "Open Tickets",
                    value: system_stats.open_tickets.to_string(),
                    icon: "🎫",
                }
            }

            // User management section
            div { class: "bg-white rounded-lg shadow overflow-hidden mb-8",
                div { class: "p-4 border-b flex justify-between items-center",
                    h2 { class: "text-xl font-semibold", "User Management" }
                    Link {
                        to: Route::AdminUserCreate {},
                        class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm",
                        "Add User"
                    }
                }
                table { class: "min-w-full divide-y divide-gray-200",
                    thead { class: "bg-gray-50",
                        tr {
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Name"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Email"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Role"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Last Active"
                            }
                            th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Actions"
                            }
                        }
                    }
                    tbody { class: "bg-white divide-y divide-gray-200",
                        for user in users.iter() {
                            UserRow { user: user.clone() }
                        }
                    }
                }
            }

            // System settings section
            div { class: "bg-white rounded-lg shadow p-6",
                h2 { class: "text-xl font-semibold mb-4", "System Settings" }
                div { class: "space-y-4",
                    // SettingToggle {
                    //     label: "Maintenance Mode",
                    //     description: "Enable to restrict access to admins only",
                    //     enabled: false,
                    //     on_toggle: None,
                    // }
                    // SettingToggle {
                    //     label: "New User Signups",
                    //     description: "Allow new users to register",
                    //     enabled: true,
                    //     on_toggle: None,
                    // }
                    div {
                        button { class: "px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700",
                            "Clear Cache"
                        }
                    }
                }
            }
        }
    }
}

// Components
#[component]
fn StatCard(title: &'static str, value: String, icon: &'static str) -> Element {
    rsx! {
        div { class: "bg-white p-4 rounded-lg shadow",
            div { class: "flex items-center",
                span { class: "text-2xl mr-3", "{icon}" }
                div {
                    p { class: "text-gray-500 text-sm", "{title}" }
                    p { class: "text-2xl font-bold", "{value}" }
                }
            }
        }
    }
}

#[component]
fn UserRow(user: AdminUser) -> Element {
    rsx! {
        tr {
            td { class: "px-6 py-4 whitespace-nowrap",
                div { class: "flex items-center",
                    div { class: "flex-shrink-0 h-10 w-10 bg-gray-200 rounded-full flex items-center justify-center",
                        {user.name.chars().next().unwrap_or('?').to_string()}
                    }
                    div { class: "ml-4",
                        div { class: "text-sm font-medium text-gray-900", "{user.name}" }
                    }
                }
            }
            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{user.email}" }
            td { class: "px-6 py-4 whitespace-nowrap",
                {
                    let role_class = match user.role.as_str() {
                        "Admin" => "bg-purple-100 text-purple-800",
                        "Manager" => "bg-blue-100 text-blue-800",
                        _ => "bg-gray-100 text-gray-800",
                    };
                    rsx! {
                        span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full {role_class}",
                            "{user.role}"
                        }
                    }
                }
            }
            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500", "{user.last_active}" }
            td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium",
                // Link {
                //     to: Route::AdminUserEdit(user.id),
                //     class: "text-blue-600 hover:text-blue-900 mr-3",
                //     "Edit"
                // }
                button {
                    class: "text-red-600 hover:text-red-900",
                    onclick: move |_| log::info!("Delete user {}", user.id),
                    "Delete"
                }
            }
        }
    }
}



// Data structures
#[derive(Clone, PartialEq, Eq)]
struct AdminUser {
    id: u64,
    name: String,
    email: String,
    role: String,
    last_active: String,
}

struct AdminStats {
    total_users: u32,
    active_today: u32,
    storage_used: String,
    open_tickets: u32,
}