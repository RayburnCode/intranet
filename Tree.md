<!-- @format -->

src/
├── components/
│ ├── layout/
│ │ ├── Header.rsx
│ │ ├── Sidebar.rsx
│ │ └── Footer.rsx
│ └── widgets/
│ ├── AnnouncementCard.rsx
│ └── QuickLink.rsx
│
├── pages/
│ ├── home.rsx # /
│ ├── directory/
│ │ ├── mod.rs # /directory
│ │ ├── org_chart.rsx
│ │ └── search.rsx
│ ├── resources/
│ │ ├── mod.rs # /resources
│ │ ├── policies.rsx
│ │ └── training.rsx
│ ├── tools/
│ │ ├── mod.rs # /tools
│ │ ├── tickets.rsx
│ │ └── calendar.rsx
│ ├── departments/
│ │ ├── mod.rs # /departments
│ │ ├── hr.rsx
│ │ └── it.rsx
│ ├── social/
│ │ ├── mod.rs # /social
│ │ ├── forums.rsx
│ │ └── kudos.rsx
│ └── admin/
│ └── mod.rs # /admin (RBAC)
│
└── app.rsx # Main router
