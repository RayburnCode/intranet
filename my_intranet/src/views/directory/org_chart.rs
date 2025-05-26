use dioxus::prelude::*;


// Data structures
#[derive(Clone, PartialEq, Eq)]
struct Team {
    name: String,
    members: Vec<Employee>,
}

#[derive(Clone, PartialEq, Eq)]
struct Employee {
    name: String,
    title: String,
    department: String,
    phone_number: Option<String>,
    email: Option<String>,
    avatar: String,
}

#[component]
fn TeamSection(team: Team) -> Element {
    rsx! {
        div {
            h3 { class: "font-medium text-lg text-blue-600 mb-2", "{team.name}" }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                for member in team.members.iter() {
                    EmployeeCard { employee: member.clone() }
                }
            }
        }
    }
}

#[component]
fn EmployeeCard(employee: Employee) -> Element {
    rsx! {
        div { class: "flex items-center p-3 border rounded-lg hover:bg-gray-50 transition",
            img {
                class: "w-12 h-12 rounded-full mr-4 object-cover",
                src: "{employee.avatar}",
                alt: "{employee.name}",
            }
            div {
                p { class: "font-medium text-gray-800", "{employee.name}" }
                p { class: "text-sm text-gray-500", "{employee.title}" }
                p { class: "text-xs text-gray-400", "{employee.department}" }
                if let Some(phone) = &employee.phone_number {
                    p { class: "text-xs text-gray-400", "Phone: {phone}" }
                }
                if let Some(email) = &employee.email {
                    p { class: "text-xs text-gray-400", "Email: {email}" }
                }
            }
        }
    }
}

#[component]
pub fn OrgChartView() -> Element {
    // Sample org chart data
    let teams = vec![
        Team {
            name: "Leadership".to_string(),
            members: vec![
                Employee {
                    name: "Sarah Johnson".to_string(),
                    title: "CEO".to_string(),
                    department: "Executive".to_string(),
                    avatar: "/avatars/sarah.png".to_string(),
                    email: Some("test@test.com".to_string()),
                    phone_number: Some("111-111-1111".to_string()),

                },
                Employee {
                    name: "Michael Chen".to_string(),
                    title: "CTO".to_string(),
                    department: "Technology".to_string(),
                    avatar: "/avatars/michael.png".to_string(),
                    email: Some("test@test.com".to_string()),
                    phone_number: Some("111-111-1111".to_string()),
                },
            ],
        },
        Team {
            name: "Team Members".to_string(),
            members: vec![
                Employee {
                    name: "Alex Rivera".to_string(),
                    title: "Senior Developer".to_string(),
                    department: "Engineering".to_string(),
                    avatar: "/avatars/alex.png".to_string(),
                    email: Some("test@test.com".to_string()),
                    phone_number: Some("111-111-1111".to_string()),
                },
                Employee {
                    name: "Jamie Smith".to_string(),
                    title: "QA Engineer".to_string(),
                    department: "Engineering".to_string(),
                    avatar: "/avatars/jamie.png".to_string(),
                    email: Some("test@test.com".to_string()),
                    phone_number: Some("111-111-1111".to_string()),
                },
            ],
        },
        
    ];

    rsx! {
        div { class: "bg-white rounded-lg shadow p-6",
            h2 { class: "text-xl font-semibold text-gray-700 mb-4", "Organization Chart" }
            div { class: "space-y-6",
                for team in teams.iter() {
                    TeamSection { team: team.clone() }
                }
            }
        }
    }
}
