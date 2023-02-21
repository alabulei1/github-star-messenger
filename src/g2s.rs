use github_flows::{listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event("jetjinser", "github-flows", vec!["issues"], handler).await;
}

async fn handler(payload: EventPayload) {
    if let EventPayload::IssuesEvent(e) = payload {
        let issue = e.issue;

        let sender = issue.user.login;
        let body = issue.body.unwrap_or_default();
        let title = issue.title;
        let url = issue.url.to_string();

        let text = format!("{}: {}\n{}\n> {}", sender, title, body, url);

        send_message_to_channel("ham-5b68442", "general", text)
    }
}
