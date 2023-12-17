mod config;
mod teamspeak;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load_config()?;

    let mut who_client = teamspeak::WhoClient::init(config.server, config.password).await?;

    let channels = who_client.get_channel_names()?;

    for client in who_client.get_clients()? {
        println!(
            "{} ({})",
            client.name,
            channels.get(&client.channel).unwrap()
        );
    }

    who_client.disconnect().await;

    Ok(())
}
