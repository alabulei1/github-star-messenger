use anyhow::{Error, Result};
use github_flows::get_octo;
use sendgrid_flows::{send_email, Email};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let octocrab = get_octo(None);

    let repo = octocrab.repos("jaykchen", "vitesse-lite").get().await?;

    let stargazers_count = repo.stargazers_count.unwrap_or(0);
    send_message_to_channel("ik8", "general", stargazers_count.to_string());
    let text = format!("You have been rewarded {stargazers_count} stars!");

    if stargazers_count % 10 == 0 {
        send_message_to_channel("jaykchen", "ik8", text.clone());
        let email = Email {
            to: vec![String::from("jaykchen@gmail.com")],
            subject: String::from("Hi"),
            content: text,
        };
        send_email("jaykchen@gmail.com", &email).expect("failed to send email");
    }

    Ok(())
}
