use dioxus::prelude::*;

#[component]
pub fn Departments() -> Element {
    let departments = vec![
        Department {
            name: "Human Resources".to_string(),
            description: "Employee relations, benefits, and company policies".to_string(),
            icon: "👥".to_string(),
            route: "/departments/hr",
        },
        Department {
            name: "Information Technology".to_string(),
            description: "Tech support, systems, and infrastructure".to_string(),
            icon: "💻".to_string(),
            route: "/departments/it",
        },
        Department {
            name: "Finance".to_string(),
            description: "Accounting, payroll, and financial reporting".to_string(),
            icon: "💰".to_string(),
            route: "/departments/finance",
        },
        Department {
            name: "Operations".to_string(),
            description: "Facilities, logistics, and office management".to_string(),
            icon: "🏭".to_string(),
            route: "/departments/operations",
        },
        Department {
            name: "Marketing".to_string(),
            description: "Branding, communications, and campaigns".to_string(),
            icon: "📢".to_string(),
            route: "/departments/marketing",
        },
        Department {
            name: "Engineering".to_string(),
            description: "Product development and technical innovation".to_string(),
            icon: "⚙️".to_string(),
            route: "/departments/engineering",
        },
    ];

    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            h1 { class: "text-3xl font-bold text-gray-800 mb-6", "Departments" }
            p { class: "text-gray-600 mb-8", "Browse company departments and access their resources" }
            // Responsive grid - 1 column on mobile, 2 on tablet, 3 on desktop
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                for dept in departments.iter() {
                    DepartmentCard { department: dept.clone() }
                }
            }
        }
    }
}

#[component]
fn DepartmentCard(department: Department) -> Element {
    rsx! {
        Link {
            to: department.route,
            class: "group block p-6 border rounded-lg hover:border-blue-500 hover:shadow-md transition-all duration-200",
            div { class: "flex items-start",
                span { class: "text-3xl mr-4", "{department.icon}" }
                div {
                    h3 { class: "text-xl font-semibold text-gray-800 group-hover:text-blue-600 mb-1",
                        "{department.name}"
                    }
                    p { class: "text-gray-600 text-sm", "{department.description}" }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Department {
    name: String,
    description: String,
    icon: String,
    route: &'static str,
}