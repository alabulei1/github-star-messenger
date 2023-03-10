use github_flows::{listen_to_event, EventPayload};
use sendgrid_flows::{send_email, Email};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("alabulei1", "a-test", vec!["star"], handler).await;

    Ok(())
}

async fn handler(payload: EventPayload) {
    let your_repo_name: &str = "vitesse-lite";

    if let EventPayload::UnknownEvent(e) = payload {
        let stargazers_count = e["repository"]["stargazers_count"].as_i64().unwrap_or(-1);

        let text = format!(
            "Congratulations on your repository {your_repo_name} with {stargazers_count} stars."
        );

        if stargazers_count % 10 == 0 {
            let email = Email {
                to: vec![String::from("vivian@secondstate.io")],
                subject: String::from("A new Star"),
                content: text,
            };
            send_email("vivian@secondstate.io", &email).expect("failed to send email");
        }
    }
}
