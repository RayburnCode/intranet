use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn AdminUserEdit(id: u64) -> Element {
    // Sample user data - replace with real data fetch
    let user = use_signal(|| User {
        id,
        name: "John Doe".to_string(),
        email: "john@company.com".to_string(),
        role: "Manager".to_string(),
        is_active: true,
    });

    let mut name_input = use_signal(|| user().name.clone());
    let mut email_input = use_signal(|| user().email.clone());
    let mut role_input = use_signal(|| user().role.clone());
    let mut is_active = use_signal(|| user().is_active);

    let save_changes = move |_| {
        // In a real app, you would call an API here
        log::info!("Saving changes for user {}", id);
    };

    rsx! {
        div { class: "p-6 max-w-3xl mx-auto",
            // Header with back button
            div { class: "flex items-center mb-6",
                Link {
                    to: Route::Admin {},
                    class: "mr-4 text-blue-600 hover:text-blue-800",
                    "← Back to Admin"
                }
                h1 { class: "text-2xl font-bold", "Edit User" }
            }

            // User edit form
            div { class: "bg-white rounded-lg shadow p-6",
                div { class: "space-y-4",
                    // Name field
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Full Name"
                        }
                        input {
                            class: "w-full p-2 border rounded",
                            value: name_input(),
                            oninput: move |e| name_input.set(e.value()),
                        }
                    }

                    // Email field
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Email"
                        }
                        input {
                            class: "w-full p-2 border rounded",
                            r#type: "email",
                            value: email_input(),
                            oninput: move |e| email_input.set(e.value()),
                        }
                    }

                    // Role dropdown
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Role"
                        }
                        select {
                            class: "w-full p-2 border rounded",
                            value: role_input(),
                            onchange: move |e| role_input.set(e.value()),
                            option { value: "Admin", "Admin" }
                            option { value: "Manager", "Manager" }
                            option { value: "User", "User" }
                        }
                    }

                    // Active status toggle
                    div { class: "flex items-center justify-between py-3",
                        div {
                            label { class: "font-medium text-gray-900", "Account Active" }
                            p { class: "text-sm text-gray-500",
                                "Disabling will prevent user from logging in"
                            }
                        }
                        button {
                            class: {
                                format!(
                                    "relative inline-flex flex-shrink-0 h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 {}",
                                    if is_active() { "bg-blue-600" } else { "bg-gray-200" },
                                )
                            },
                            role: "switch",
                            aria_checked: is_active(),
                            onclick: move |_| is_active.toggle(),
                            span {
                                class: {
                                    format!(
                                        "inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {}",
                                        if is_active() { "translate-x-6" } else { "translate-x-1" },
                                    )
                                },
                                aria_hidden: "true",
                            }
                        }
                    }

                    // Action buttons
                    div { class: "flex justify-end space-x-3 pt-4",
                        Link {
                            to: Route::Admin {},
                            class: "px-4 py-2 border rounded text-gray-700 hover:bg-gray-50",
                            "Cancel"
                        }
                        button {
                            class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                            onclick: save_changes,
                            "Save Changes"
                        }
                    }
                }
            }
        }
    }
}

// User data structure
#[derive(Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
    role: String,
    is_active: bool,
}