// /admin (RBAC)


// pages/admin/mod.rs
#[component]
fn AdminPortal() -> Element {
    let user = use_user_context();
    
    // Guard clause
    if !user.is_admin {
        return rsx! { "Unauthorized" };
    }

    rsx! {
        div { class: "admin-portal",
            UserManagement {}
            AnalyticsDashboard {}
        }
    }
}



#[component]
fn ProtectedRoute(allowed_roles: Vec<Role>, children: Element) -> Element {
    let user = use_user_context();
    if user.has_any_role(&allowed_roles) {
        children
    } else {
        rsx! {
            Redirect { to: "/unauthorized" }
        }
    }
}