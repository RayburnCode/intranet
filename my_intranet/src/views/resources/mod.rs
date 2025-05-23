use dioxus::prelude::*;

#[component]
pub fn Resources() -> Element {
    let resource_categories = vec![
        ResourceCategory {
            name: "Company Policies".to_string(),
            icon: "📜".to_string(),
            resources: vec![
                Resource {
                    title: "Employee Handbook".to_string(),
                    description: "Complete guide to company policies".to_string(),
                    link: "/resources/handbook",
                    file_type: "PDF".to_string(),
                },
                Resource {
                    title: "Remote Work Policy".to_string(),
                    description: "Guidelines for hybrid work arrangements".to_string(),
                    link: "/resources/remote-policy",
                    file_type: "PDF".to_string(),
                },
            ],
        },
        ResourceCategory {
            name: "Training Materials".to_string(),
            icon: "🎓".to_string(),
            resources: vec![
                Resource {
                    title: "Security Awareness".to_string(),
                    description: "Annual cybersecurity training".to_string(),
                    link: "/resources/security-training",
                    file_type: "Video".to_string(),
                },
                Resource {
                    title: "DEI Training".to_string(),
                    description: "Diversity, equity and inclusion resources".to_string(),
                    link: "/resources/dei-training",
                    file_type: "Course".to_string(),
                },
            ],
        },
        ResourceCategory {
            name: "Templates".to_string(),
            icon: "📝".to_string(),
            resources: vec![
                Resource {
                    title: "Expense Report".to_string(),
                    description: "Standard expense submission form".to_string(),
                    link: "/resources/expense-template",
                    file_type: "XLSX".to_string(),
                },
                Resource {
                    title: "Project Charter".to_string(),
                    description: "Template for new project proposals".to_string(),
                    link: "/resources/project-template",
                    file_type: "DOCX".to_string(),
                },
            ],
        },
    ];

    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            h1 { class: "text-3xl font-bold text-gray-800 mb-2", "Company Resources" }
            p { class: "text-gray-600 mb-8",
                "Access important documents, templates, and training materials"
            }
            // Search bar
            div { class: "mb-8",
                div { class: "relative max-w-xl",
                    input {
                        class: "w-full p-3 pl-10 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                        placeholder: "Search resources...",
                        r#type: "search",
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
            }
            // Categories
            div { class: "space-y-8",
                for category in resource_categories.iter() {
                    ResourceCategorySection { category: category.clone() }
                }
            }
        }
    }
}

#[component]
fn ResourceCategorySection(category: ResourceCategory) -> Element {
    rsx! {
        div {
            div { class: "flex items-center mb-4",
                span { class: "text-2xl mr-3", "{category.icon}" }
                h2 { class: "text-xl font-semibold text-gray-800", "{category.name}" }
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                for resource in category.resources.iter() {
                    ResourceCard { resource: resource.clone() }
                }
            }
        }
    }
}

#[component]
fn ResourceCard(resource: Resource) -> Element {
    rsx! {
        Link {
            to: resource.link,
            class: "group block p-5 border rounded-lg hover:bg-blue-50 transition-colors",
            div { class: "flex justify-between items-start mb-2",
                h3 { class: "text-lg font-medium text-gray-800 group-hover:text-blue-600",
                    "{resource.title}"
                }
                span { class: "text-xs px-2 py-1 bg-gray-100 text-gray-600 rounded",
                    "{resource.file_type}"
                }
            }
            p { class: "text-gray-600 text-sm", "{resource.description}" }
            div { class: "mt-3 text-sm text-blue-500 flex items-center",
                "View Resource"
                svg {
                    class: "ml-1 h-4 w-4",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M9 5l7 7-7 7",
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct ResourceCategory {
    name: String,
    icon: String,
    resources: Vec<Resource>,
}

#[derive(Clone, PartialEq, Eq)]
struct Resource {
    title: String,
    description: String,
    link: &'static str,
    file_type: String,
}