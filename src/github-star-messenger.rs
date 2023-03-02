use github_flows::{get_octo, listen_to_event, EventPayload};
use sendgrid_flows::{send_email, Email};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("jaykchen", "vitesse-lite", vec!["star"], handler).await;

    Ok(())
}

async fn handler(payload: EventPayload) {
    let your_github_login: &str = "jaykchen";
    let your_repo_name: &str = "vitesse-lite";

    // let octocrab = get_octo(None);

    // let repo = octocrab
    //     .repos(your_github_login, your_repo_name)
    //     .get()
    //     .await
    //     .expect("repo not obtained");

    // let stargazers_count = repo.stargazers_count.unwrap_or(0);
    if let EventPayload::UnknownEvent(e) = payload {
        let repo = e.get("repository").expect("repo not obtained");
        let stargazers_count = repo["stargazers_count"].as_i64().unwrap_or(0);

        let text = format!(
            "Congratulations on your repository {your_repo_name} with {stargazers_count} stars."
        );

        if stargazers_count % 10 == 0 {
            let email = Email {
                to: vec![String::from("jaykchen@gmail.com")],
                subject: String::from("Hi"),
                content: text,
            };
            send_email("jaykchen@gmail.com", &email).expect("failed to send email");
        }
    }
}
