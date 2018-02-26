use std::thread;
use std::time::Duration;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate dbus;
use dbus::*;

extern crate mqtt;
use mqtt::control::variable_header::ConnectReturnCode;

mod mqtt_session;
use mqtt_session::MqttSession;
mod mqtt_config;
mod location;
use location::Location;

fn main() {
    let c = Connection::get_private(BusType::Session).unwrap(); // D-Bus init
    println!("---");
    println!("master provider: {}",   location::provider_name(&c));
    println!("location provider: {}", location::addr_provider_name(&c));
    println!("position provider: {}", location::position_provider_name(&c));
    println!("---");

    let conf = mqtt_config::get("mqtt-auth.json"); // this file is not in version control
    let mut session = MqttSession::new(conf.server);
    session.connect(conf.client_id, conf.username, conf.password);

    let ack = session.read_ack().unwrap();
    if ack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
        panic!("Failed to connect to server, return code {:?}", ack.connect_return_code());
    }

    loop { // send location periodically
        let location = Location::get_address(&c)
                                .unwrap_or(Location::empty())
                                .merge(Location::get_position(&c)
                                .unwrap_or(Location::empty()))
                                .as_json().unwrap();
        println!("{}", location);
        session.publish(conf.topic.clone(), location);
        thread::sleep(Duration::from_secs(conf.send_period.unwrap_or(10)));
    }
}
