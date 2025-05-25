use dioxus::prelude::*;


#[component]
pub fn SettingToggle(
    label: &'static str, 
    description: &'static str, 
    enabled: bool,
    on_toggle: Option<EventHandler<bool>>,  // Optional callback for parent component
) -> Element {
    let mut is_enabled = use_signal(|| enabled);

    // Handle toggle changes
    let handle_toggle = move |_| {
        let new_state = !is_enabled();
        is_enabled.set(new_state);
        if let Some(handler) = &on_toggle {
            handler.call(new_state);
        }
    };

    rsx! {
        div { class: "flex items-center justify-between py-3",
            div { class: "flex flex-col",
                p {
                    class: "font-medium text-gray-900",
                    aria_label: format_args!("{} toggle", label),
                    "{label}"
                }
                p { class: "text-sm text-gray-500", "{description}" }
            }
            button {
                class: {
                    if is_enabled() {
                        "relative inline-flex flex-shrink-0 h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 bg-blue-600"
                    } else {
                        "relative inline-flex flex-shrink-0 h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 bg-gray-200"
                    }
                },
                role: "switch",
                aria_checked: is_enabled(),
                onclick: handle_toggle,
                // Hidden input for form compatibility
                input {
                    r#type: "checkbox",
                    class: "sr-only",
                    checked: is_enabled(),
                    readonly: true,
                }
                span {
                    class: {
                        if is_enabled() {
                            "inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out translate-x-6"
                        } else {
                            "inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out translate-x-1"
                        }
                    },
                    aria_hidden: "true",
                }
            }
        }
    }
}