use github_flows::{get_octo, listen_to_event, EventPayload};
use sendgrid_flows::{send_email, Email};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let your_github_login: &str = "jaykchen";
    let your_repo_name: &str = "vitesse-lite";

    listen_to_event(your_github_login, your_repo_name, vec!["star"], handler).await;

    let octocrab = get_octo(None);

    let repo = octocrab
        .repos(your_github_login, your_repo_name)
        .get()
        .await?;

    let stargazers_count = repo.stargazers_count.unwrap_or(0);

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

    Ok(())
}

async fn handler(payload: EventPayload) {
    let text = format!("{:?}", payload);
    println!("{text}");
}
