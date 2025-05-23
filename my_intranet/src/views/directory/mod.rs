// pages/directory/mod.rs
#[component]
fn Directory() -> Element {
    rsx! {
        div { class: "directory-page",
            OrgChartView {}
            EmployeeSearch { filters: ["Department", "Skills"] }
        }
    }
}