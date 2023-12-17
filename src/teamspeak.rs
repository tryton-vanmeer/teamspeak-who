use anyhow::{bail, Result};
use futures::prelude::*;
use tokio::time::{self, Duration};
use tsclientlib::{Connection, DisconnectOptions, OutCommandExt, StreamItem, data::Client};

pub struct WhoClient {
    connection: Connection,
}

impl WhoClient {
    pub async fn init(address: String, password: String) -> Result<WhoClient> {
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

        Ok(WhoClient { connection })
    }

    pub async fn disconnect(&mut self) {
        self.connection
            .disconnect(DisconnectOptions::new())
            .unwrap();
        self.connection
            .events()
            .for_each(|_| future::ready(()))
            .await;
    }

    pub fn get_clients(&self) -> Result<Vec<&Client>> {
        let mut clients: Vec<_> = self.connection.get_state()?.clients.values().collect();
        clients.sort_by_key(|c| c.channel.0);

        Ok(clients)
    }
}
