use anyhow::{bail, Result};
use futures::prelude::*;
use tokio::time::{self, Duration};
use tsclientlib::{Connection, OutCommandExt, StreamItem, DisconnectOptions};

pub async fn connect(address: String, password: String) -> Result<Connection> {
    let mut connection = Connection::build(address)
        .version(tsclientlib::Version::Linux_3_X_X)
        .password(password)
        .name("teamspeak-who")
        .connect()?;

    connection
        .events()
        .try_filter(|e| future::ready(matches!(e, StreamItem::BookEvents(_))))
        .next()
        .await
        .unwrap()?;

    connection
        .get_state()?
        .server
        .set_subscribed(true)
        .send(&mut connection)?;

    let mut events = connection.events().try_filter(|_| future::ready(false));
    tokio::select! {
        _ = time::sleep(Duration::from_secs(1)) => {}
        _ = events.next() => {
            bail!("Disconnected");
        }
    };
    drop(events);

    Ok(connection)
}

pub async fn disconnect(connection: &mut Connection) {
    connection.disconnect(DisconnectOptions::new()).unwrap();
    connection.events().for_each(|_| future::ready(())).await;
}
