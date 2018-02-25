use std::io::{Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate dbus;
use dbus::*;

extern crate mqtt;
use mqtt::{Decodable, Encodable};
use mqtt::packet::*;
use mqtt::TopicName;
use mqtt::control::variable_header::ConnectReturnCode;

mod mqtt_config;
mod location;
use location::Location;

fn main() {
    let c = Connection::get_private(BusType::Session).unwrap(); // D-Bus init
    println!("provider: {}", location::provider_name(&c));
    println!("location: {}", location::addr_provider_name(&c));
    println!("position: {}", location::position_provider_name(&c));

    let mut buf    = Vec::new();
    let     conf   = mqtt_config::get("mqtt-auth.json"); // not in version control
    let mut stream = TcpStream::connect(conf.server).unwrap(); // MQTT init
    let mut conn   = ConnectPacket::new("MQTT", conf.client_id);
    conn.set_clean_session(true);
    conn.set_user_name(conf.username);
    conn.set_password(conf.password);
    conn.encode(&mut buf).unwrap();
    stream.write_all(&buf).unwrap();

    let ack = ConnackPacket::decode(&mut stream).unwrap();
    if ack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
        panic!("Failed to connect to server, return code {:?}", ack.connect_return_code());
    }

    loop {
        buf.clear();
        let location = Location::get_address(&c).unwrap_or(Location::empty())
                                .merge(Location::get_position(&c).unwrap_or(Location::empty()))
                                .as_json().unwrap();
        println!("{}", location);

        let publish = PublishPacket::new(TopicName::new(conf.topic.clone()).unwrap(),
                                    QoSWithPacketIdentifier::Level0,
                                    location);
        publish.encode(&mut buf).unwrap();
        stream.write_all(&buf).unwrap();
        thread::sleep(Duration::from_secs(10));
    }
}
