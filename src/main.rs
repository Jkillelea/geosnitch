use std::thread;
use std::time::Duration;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate dbus;
use dbus::*;

extern crate mqtt;
use mqtt::control::variable_header::ConnectReturnCode;

mod mqtt_session;
mod mqtt_config;
mod location;
use mqtt_session::MqttSession;
use location::Location;

fn main() {
    let c = Connection::get_private(BusType::Session).unwrap(); // D-Bus init

    let providers = ["Geonames", "Localnet", "Skyhook", "UbuntuGeoIP"];
    for p in providers.iter() {
        let dest = format!("org.freedesktop.Geoclue.Providers.{}", p);
        let path = format!("/org/freedesktop/Geoclue/Providers/{}", p);
        println!("{}: {:#?}", p, location::provider_status(&c, &dest, &path));
    }

    println!("---");
    println!("clients: {:?}", location::available_clients(&c));
    println!("master provider: {}",   location::provider_name(&c));
    println!("location provider: {}", location::addr_provider_name(&c));
    println!("position provider: {}", location::position_provider_name(&c));
    println!("---");

    let conf = mqtt_config::get(&"mqtt-auth.json"); // this file is not in version control
    let mut mqtt_session = MqttSession::new(conf.server);
    mqtt_session.connect(conf.client_id, conf.username, conf.password);

    let ack = mqtt_session.read_ack().unwrap();
    assert_eq!(ack.connect_return_code(), ConnectReturnCode::ConnectionAccepted);

    loop { // send location periodically
        let location = Location::get_address(&c)
                                .unwrap_or(Location::empty())
                                .merge(Location::get_position(&c)
                                .unwrap_or(Location::empty()))
                                .as_json().unwrap();
        println!("{}", location);
        mqtt_session.publish(conf.topic.clone(), location);
        thread::sleep(Duration::from_secs(conf.send_period.unwrap_or(10)));
    }
}


