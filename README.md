<div align="center">
  <h1>TeamSpeak Who</h1>
  Show the online clients on a TeamSpeak server like <code>who(1)</code>
</div>

## Usage

Ensure the config file (see below) exists and just run `teamspeak-who`.

## Config

Create `teamspeak-who/config.toml` in the XDG config directory. Running without
the config will produce an error showing where it looks for the file.

Ex. /home/username/.config/teamspeak-who/config.toml.

```toml
server = "the address of the server"
channel = "the name of the channel to check"
password = "password to connect"
```
