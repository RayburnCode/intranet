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

### Intranet use cases

Local First: Cache critical docs/tools.

Quick-access tools (tickets, forms, directories)

1. Announcements & News Feed

- Company-wide updates (priority tagging: urgent, general)
- Leadership messages (video/text)

2. Employee Directory

- Searchable org chart
- Contact details (email, phone, Slack/MS Teams)

3. Calendar & Events

4. Internal Forums or Wikis

- Q&A boards (Stack Overflow-style)
- Knowledge base (FAQs, troubleshooting)
- Best practices documentation

5. IT Support Center

- System status (outage alerts)
- Software request forms
- Password reset guides

6. Policy Repository

- Company handbook
- Compliance training (GDPR, HIPAA)
- Mandatory acknowledgments (e.g., security policies

7. Social & Engagement

- Pulse surveys (e.g., "How’s remote work?")
- Reviews of LO's and Processors
