use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};

use mqtt;
use mqtt::{Encodable, Decodable};
use mqtt::TopicName;
use mqtt::packet::*;

pub struct MqttSession {
    stream: TcpStream,
}

impl MqttSession {
    pub fn new<Addr: ToSocketAddrs>(server: Addr) -> MqttSession {
        trace!("mqtt_session::MqttSession::new");
        MqttSession {
            stream: TcpStream::connect(server).unwrap(),
        }
    }

    pub fn connect(&mut self, client_id: String, username: Option<String>, password: Option<String>) {
        trace!("mqtt_session::MqttSession::connect");
        let mut conn = ConnectPacket::new("MQTT", client_id);
        conn.set_clean_session(true);
        conn.set_user_name(username);
        conn.set_password(password);
        send(&conn, &mut self.stream);
    }

    pub fn publish<Data: Into<Vec<u8>>>(&mut self, topic: String, data: Data) {
        trace!("mqtt_session::MqttSession::publish");
        let publish = PublishPacket::new(TopicName::new(topic).unwrap(),
                                        QoSWithPacketIdentifier::Level0,
                                        data);
        send(&publish, &mut self.stream);
    }

    pub fn read_ack(&mut self) -> Result<ConnackPacket, PacketError<mqtt::packet::ConnackPacket>> {
        trace!("mqtt_session::MqttSession::read_ack");
        ConnackPacket::decode(&mut self.stream)
    }
}

fn send<'a, Pack: Encodable<'a>>(packet: &'a Pack, stream: &mut TcpStream) {
    trace!("mqtt_session::MqttSession::send");
    let mut buf = Vec::new();
    packet.encode(&mut buf).unwrap();
    stream.write_all(&buf).unwrap();
}
