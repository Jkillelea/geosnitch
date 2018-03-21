use std::thread;
use std::time::Duration;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate dbus;
extern crate mqtt;
use mqtt::control::variable_header::ConnectReturnCode;

mod mqtt_session;
mod mqtt_config;
mod location;
mod location_session;
use mqtt_session::MqttSession;
use location::Location;
use location_session::LocationSession;

fn main() {
    let session = LocationSession::new();

    let providers = ["Geonames", "Localnet", "Skyhook", "UbuntuGeoIP"];
    for p in providers.iter() {
        let dest = format!("org.freedesktop.Geoclue.Providers.{}", p);
        let path = format!("/org/freedesktop/Geoclue/Providers/{}", p);
        println!("{}: {:#?}", p, session.provider_status(&dest, &path));
    }

    println!("---");
    println!("master provider: {}",   session.provider_name());
    println!("location provider: {}", session.addr_provider_name());
    println!("position provider: {}", session.position_provider_name());
    println!("---");

    let conf = mqtt_config::get(&"mqtt-auth.json"); // this file is not in version control
    let mut mqtt_session = MqttSession::new(conf.server);
    mqtt_session.connect(conf.client_id, conf.username, conf.password);

    let ack = mqtt_session.read_ack().unwrap();
    assert_eq!(ack.connect_return_code(), ConnectReturnCode::ConnectionAccepted);

    loop { // send location periodically
        let location = session.get_address()
                              .unwrap_or(Location::empty())
                              .merge(session.get_position()
                              .unwrap_or(Location::empty()))
                              .as_json().unwrap();
        println!("{}", location);
        mqtt_session.publish(conf.topic.clone(), location);
        thread::sleep(Duration::from_secs(conf.send_period.unwrap_or(10)));
    }
}


