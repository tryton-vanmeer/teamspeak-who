mod config;
mod teamspeak;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load_config()?;

    let mut connection = teamspeak::connect(config.server, config.password).await?;

    let state = connection.get_state()?;

    let &channel = state
        .channels
        .iter()
        .find_map(|(key, val)| {
            if val.name == config.channel {
                Some(key)
            } else {
                None
            }
        })
        .unwrap();

    for client in state.clients.values() {
        if client.channel == channel {
            println!("{}", client.name);
        }
    }

    teamspeak::disconnect(&mut connection).await;

    Ok(())
}
