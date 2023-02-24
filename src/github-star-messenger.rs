use anyhow::{Error, Result};
use github_flows::{get_octo, listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    // listen_to_event("jaykchen", "vitesse-lite", vec!["star"], handler).await;
    let octo = get_octo(None);
    let repo = octo.repos("jaykchen", "vitesse-lite").get().await?;

    let full_name = repo.full_name.unwrap();
    let stargazers_count = repo.stargazers_count.unwrap();
    let text = format!(
        "Congratulations on your repository {} with {} stars.",
        full_name, stargazers_count
    );
    send_message_to_channel("jaykchen", "ik8", text);

    Ok(())
}

// async fn handler(payload: EventPayload) {
//     if let EventPayload::UnknownEvent(e) = payload {
//         let repo = e.get("repository").unwrap();
//         send_message_to_channel("jaykchen", "ik8", repo.to_string());

//         let full_name = repo["full_name"].as_str().unwrap();
//         let stargazers_count = repo["stargazers_count"].as_i64().unwrap();
//         let text = format!(
//             "Congratulations on your repository {} with {} stars.",
//             full_name, stargazers_count
//         );
//         send_message_to_channel("jaykchen", "ik8", text.clone());

//         if stargazers_count % 10 == 0 {
//             send_message_to_channel("jaykchen", "ik8", text)
//         }
//     }
// }
