use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn AdminUserCreate() -> Element {
    // Form state
    let mut name_input = use_signal(|| String::new());
    let mut email_input = use_signal(|| String::new());
    let mut password_input = use_signal(|| String::new());
    let mut confirm_password_input = use_signal(|| String::new());
    let mut role_input = use_signal(|| "User".to_string());
    let mut is_active = use_signal(|| true);

    let create_user = move |_| {
        // Validate inputs
        if password_input() != confirm_password_input() {
            log::error!("Passwords don't match!");
            return;
        }

        // In a real app, you would call an API here
        log::info!(
            "Creating user: {} ({}) with role {}",
            name_input(),
            email_input(),
            role_input()
        );
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
                h1 { class: "text-2xl font-bold", "Create New User" }
            }

            // User creation form
            div { class: "bg-white rounded-lg shadow p-6",
                div { class: "space-y-4",
                    // Name field
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Full Name"
                        }
                        input {
                            class: "w-full p-2 border rounded",
                            placeholder: "Enter full name",
                            value: name_input(),
                            oninput: move |e| name_input.set(e.value()),
                            required: true,
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
                            placeholder: "user@company.com",
                            value: email_input(),
                            oninput: move |e| email_input.set(e.value()),
                            required: true,
                        }
                    }

                    // Password field
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Password"
                        }
                        input {
                            class: "w-full p-2 border rounded",
                            r#type: "password",
                            placeholder: "••••••••",
                            value: password_input(),
                            oninput: move |e| password_input.set(e.value()),
                            required: true,
                        }
                    }

                    // Confirm Password field
                    div {
                        label { class: "block text-sm font-medium text-gray-700 mb-1",
                            "Confirm Password"
                        }
                        input {
                            class: "w-full p-2 border rounded",
                            r#type: "password",
                            placeholder: "••••••••",
                            value: confirm_password_input(),
                            oninput: move |e| confirm_password_input.set(e.value()),
                            required: true,
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
                                "User will be able to login immediately"
                            }
                        }
                        button {
                            class: {
                                let base = "relative inline-flex flex-shrink-0 h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500";
                                if is_active() {
                                    format!("{} bg-blue-600", base)
                                } else {
                                    format!("{} bg-gray-200", base)
                                }
                            },
                            role: "switch",
                            aria_checked: is_active(),
                            onclick: move |_| is_active.toggle(),
                            span {
                                class: {
                                    let base = "inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out";
                                    if is_active() {
                                        format!("{} translate-x-6", base)
                                    } else {
                                        format!("{} translate-x-1", base)
                                    }
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
                            onclick: create_user,
                            "Create User"
                        }
                    }
                }
            }
        }
    }
}