use github_flows::{listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event("jetjinser", "github-flows", vec!["issues"], handler).await;
}

async fn handler(payload: EventPayload) {
    if let EventPayload::UnknownEvent(e) = payload {
        let repo = e.get("repository").unwrap();

        let full_name = repo["full_name"].as_str().unwrap();
        let stargazers_count = repo["stargazers_count"].as_i64().unwrap();
        let text = format!(
            "Congratulations on your repository {} with {} stars.",
            full_name, stargazers_count
        );
        send_message_to_channel("jaykchen", "ik8", text.clone());

        if stargazers_count % 10 == 0 {
            send_message_to_channel("jaykchen", "ik8", text)
        }
    }
}
