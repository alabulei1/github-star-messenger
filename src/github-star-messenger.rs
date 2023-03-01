use anyhow::{Error, Result};
use github_flows::get_octo;
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let octocrab = get_octo(None);

    let repo = octocrab.repos("jaykchen", "vitesse-lite").get().await?;

    let stargazers_count = repo.stargazers_count.unwrap_or(0);
    send_message_to_channel("ik8", "general", stargazers_count.to_string());

    if stargazers_count % 10 == 0 {
        send_message_to_channel("jaykchen", "ik8", stargazers_count.to_string());
    }
    Ok(())
}
