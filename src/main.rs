mod config;
mod teamspeak;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load_config()?;

    let mut client = teamspeak::Client::init(config.server, config.password).await?;

    // let state = connection.get_state()?;

    // for client in state.clients.values() {
    //     println!("{}", client.name);
    // }

    client.disconnect().await;

    Ok(())
}
