# Geosnitch

### What is it?

A quick Rust project to track my laptop. It's still in development, but the end idea is to leave it running
as a service that reports back my laptop's location periodically.

I'm currently using GeoClue as a location provider and MQTT to report location. I'd like to make this more flexible
and modular in the future.

Other glaring flaws currently include the fact that the location data is sent over plain TCP, so anyone can read the
packets right now. Encryption would be prudent.
