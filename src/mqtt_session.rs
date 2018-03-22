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
        MqttSession {
            stream: TcpStream::connect(server).unwrap(),
        }
    }

    pub fn connect(&mut self, client_id: String, username: Option<String>, password: Option<String>) {
        let mut conn = ConnectPacket::new("MQTT", client_id);
        conn.set_clean_session(true);
        conn.set_user_name(username);
        conn.set_password(password);
        send(&conn, &mut self.stream);
    }

    pub fn publish<Data: Into<Vec<u8>>>(&mut self, topic: String, data: Data) {
        let publish = PublishPacket::new(TopicName::new(topic).unwrap(),
                                        QoSWithPacketIdentifier::Level0,
                                        data);
        send(&publish, &mut self.stream);
    }

    pub fn read_ack(&mut self) -> Result<ConnackPacket,
                                         mqtt::packet::PacketError<mqtt::packet::ConnackPacket>> {
        ConnackPacket::decode(&mut self.stream)
    }
}

fn send<'a, Pack: Encodable<'a>>(packet: &'a Pack, stream: &mut TcpStream) {
    let mut buf = Vec::new();
    packet.encode(&mut buf).unwrap();
    stream.write_all(&buf).unwrap();
}
