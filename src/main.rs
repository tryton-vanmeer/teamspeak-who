mod config;
mod teamspeak;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load_config()?;

    let mut who_client = teamspeak::WhoClient::init(config.server, config.password).await?;

    for client in who_client.get_clients()? {
        println!("{}", client.name);
    }

    who_client.disconnect().await;

    Ok(())
}
