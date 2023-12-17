mod config;
mod teamspeak;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load_config()?;

    let mut connection = teamspeak::connect(config.server, config.password).await?;

    let state = connection.get_state()?;

    for client in state.clients.values() {
        println!("{}", client.name);
    }

    teamspeak::disconnect(&mut connection).await;

    Ok(())
}
