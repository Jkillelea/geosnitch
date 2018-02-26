# Geosnitch ![](https://travis-ci.org/Jkillelea/geosnitch.svg?branch=master)

### What is it?

A quick Rust project to track my laptop. It's still in development, but the end
idea is to leave it running as a service that reports back my laptop's location
periodically to an outside server.

I'm currently using GeoClue as a location provider and MQTT to report location.
I'd like to make this more flexible and modular in the future, particularly
since GeoClue is a Linux specific D-Bus service

Other glaring flaws currently include the fact that the location data is sent over plain TCP, so anyone can read the
packets right now. Encryption would be prudent.

### Using it
You'll need to be using a Linux machine with D-Bus and GeoClue installed and running.
Create a file named `mqtt-auth.json` in the crate root with the following content

```
{
  "server":"server:port",
  "username":"account_username", (NOTE: optional)
  "password":"account_password", (NOTE: optional)
  "client_id":"client_id",
  "topic":"publish_topic",
  "send_period": 10              (NOTE: optional. Default every 10 sec)
}
```

Build and run `cargo run`
