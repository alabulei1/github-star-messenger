use anyhow::{Error, Result};
use github_flows::{get_octo, listen_to_event, EventPayload};
use sendgrid_flows::{send_email, Email};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    listen_to_event("jaykchen", "vitesse-lite", vec!["star"], handler).await;

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

async fn handler(payload: EventPayload) {
    let text = format!("{:?}", payload);
    println!("{:?}", text);
    // let mut title: String = "no title found".to_string();
    // let mut merged: bool = false;
    // let mut html_url: String = "no html_url found".to_string();
    // let mut user: String = "".to_string();

    // if let EventPayload::PullRequestEvent(e) = payload {
    //     let pr = e.pull_request;
    //     title = pr.title.expect("no title");
    //     html_url = pr.html_url.expect("no html_url field").to_string();
    //     user = pr.user.expect("user not found").login;
    //     merged = match pr.merge_commit_sha {
    //         Some(_sha) => true,
    //         None => false,
    //     };
    // }

    // get_email(&user).await;

    // match get_email(&user).await {
    //     None => {}
    //     Some(email) => {
    // let merged_str = if merged { "merged" } else { "null" };
    // let text = format!("title: {title}\n html_url: {html_url}\n user: {user}\n, email: email\n merged: {merged_str}");
    // send_message_to_channel("ik8", "general", text);
    // }
    // }
}
