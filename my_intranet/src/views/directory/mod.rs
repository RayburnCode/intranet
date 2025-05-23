// pages/directory/mod.rs
use dioxus::prelude::*;

pub mod org_chart;
pub mod search;
use org_chart::OrgChartView;
use search::EmployeeSearch;



#[component]
pub fn Directory() -> Element {
    rsx! {
        div { class: "p-6 max-w-7xl mx-auto",
            h1 { class: "text-3xl font-bold text-gray-800 mb-6", "Employee Directory" }
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                div { class: "lg:col-span-2", OrgChartView {} }
                div { class: "lg:col-span-1",
                    EmployeeSearch { filters: vec!["Department".to_string(), "Skills".to_string()] }
                }
            }
        }
    }
}








