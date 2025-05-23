<!-- @format -->

```
dx serve --platform web
```

```
npx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Using GCP

1. Create a new project in Google Cloud Console
2. Enable Cloud Run API, IAP API, and Compute Engine API

```
use axum::{http::HeaderMap, extract::Request, middleware::Next};

async fn iap_auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Verify IAP JWT header here
    // See Google's documentation for proper validation

    Ok(next.run(request).await)
}
```

# Corporate Intranet Use Cases

## 1. Local-First & Offline Access

- **Cached Resources**
  - Policy documents (PDF/HTML)
  - Emergency contact lists
  - IT troubleshooting guides
- **Sync Mechanism**
  - Background updates when online
  - Conflict resolution for offline edits

## 2. Quick-Access Tools

- **Tickets**
  - IT/HR ticket submission + attachments
  - Status tracking (Pending/Resolved)
- **Forms**
  - Dynamic forms (leave requests, orders)
  - Draft autosave
- **Directories**
  - Office maps with desk numbers
  - Vendor contacts database

## 3. Announcements & News Feed

- **Priority Tiers**
  - `Urgent` (red banners, push notifications)
  - `Department-Specific` (filterable)
- **Multimedia**
  - Embedded CEO videos
  - Audio briefings

## 4. Employee Directory

- **Advanced Features**
  - Skills tagging (e.g., "Python", "Six Sigma")
  - Real-time status (OoO/In Meeting)
- **Dark Mode**
  - For shift workers

## 5. Calendar & Events

- **Integrations**
  - Google Calendar/Outlook sync
  - Room booking with floor plans
- **Views**
  - Team timelines
  - Company milestones

## 6. Internal Forums & Wikis

- **Moderation**
  - Upvote/downvote system
  - Content flagging
- **Templates**
  - Standardized how-to guides

## 7. IT Support Center

- **Self-Service**
  - Chatbot for common issues
  - Automated password reset
- **Hardware**
  - Equipment checkout system

## 8. Policy Repository

- **Compliance**
  - Digital signature collection
  - Retraining reminders
- **Version Control**
  - Change diffs
  - Deprecated policy archive

## 9. Social & Engagement

- **Surveys**
  - Anonymous pulse checks
  - Real-time HR dashboards
- **Performance**
  - LO/Processor review templates
  - Peer "Kudos" badges

## 10. Additional Critical Use Cases

### A. Onboarding Portal

- **Checklists**
  - Day 1 tasks (email setup)
  - 30/60/90-day goals
- **Mentor Matching**
  - Auto-assign by role

### B. Analytics & Reporting

- **Metrics**
  - Pageview heatmaps
  - Search term trends
- **Custom Reports**
  - Exportable HR datasets

### C. Emergency Protocols

- **Crisis Tools**
  - One-click office closure alerts
  - Geolocated safety guides

### D. Department Hubs

- **Finance**
  - Budget dashboards
- **Legal**
  - Contract repository

---

> **Implementation Note**:  
> All features should support:
>
> - Role-based access control (RBAC)
> - Audit logging
> - Mobile responsiveness
