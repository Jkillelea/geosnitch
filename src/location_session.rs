#![allow(dead_code)]
use std::collections::HashMap;
use dbus::{self, Connection, BusType};
use location::{self, Location};

const TIMEOUT: i32 = 20; // miliseconds?
const DESTINATION: &'static str = "org.freedesktop.Geoclue.Master";

#[derive(Debug)]
pub struct LocationSession {
    connection: Connection, // dbus connection
    client_path: String,
}

impl LocationSession {
    // connect to the d bus and find the best client (last in the list)
    pub fn new() -> LocationSession {
        let conn       = Connection::get_private(BusType::Session).unwrap();
        let client_num = available_clients(&conn).pop().unwrap();

        LocationSession {
            connection: conn,
            client_path: format!("/org/freedesktop/Geoclue/Master/client{}", client_num),
        }
    }

    // Query the D-Bus for address
    pub fn get_address(&self) -> Result<Location, String> {
        let interface = "org.freedesktop.Geoclue.Address";
        let method    = "GetAddress";
        let msg       = dbus::Message::new_method_call(DESTINATION,
                                                       &self.client_path,
                                                       interface,
                                                       method)?;
        let response  = blocking_send(&self.connection, msg)?;
        // awkward argument parsing
        let (_t, loc): (i32, HashMap<&str, String>) = response.read2()
                                                      .unwrap_or((0, HashMap::new()));
        Ok(Location::from_location_hashmap(loc))
    }

    // Query the D-Bus for position
    pub fn get_position(&self) -> Result<Location, String> {
        let interface = "org.freedesktop.Geoclue.Position";
        let method    = "GetPosition";
        let msg       = dbus::Message::new_method_call(DESTINATION,
                                                       &self.client_path,
                                                       interface,
                                                       method)?;
        let response  = blocking_send(&self.connection, msg)?;
        let (_n, _t, lat, lon): (i32, i32, f64, f64) = response.read4()
                                                       .unwrap_or((0, 0, 0.0, 0.0));
        Ok(Location::from_lat_lon(lat, lon))
    }

    // -- misc things -- 
    // Some useful public functions
    // Name, Description
    pub fn provider_info(&self) -> Result<(String, String), String> {
        let interface = "org.freedesktop.Geoclue";
        let method    = "GetProviderInfo";
        let msg       = dbus::Message::new_method_call(DESTINATION,
                                                       &self.client_path,
                                                       interface,
                                                       method)?;
        let response  = blocking_send(&self.connection, msg)?;
        match response.read2() {
            Ok(data) => Ok(data),
            Err(_) => Err(String::from("dbus::arg::TypeMismatchError (dbus::Message::read2())"))
        }
    }

    // Name, Description, Service, Path
    pub fn addr_provider_info(&self) -> Result<(String, String, String, String), String> {
        let interface = "org.freedesktop.Geoclue.MasterClient";
        let method    = "GetAddressProvider";
        let msg       = dbus::Message::new_method_call(DESTINATION,
                                                       &self.client_path,
                                                       interface,
                                                       method)?;
        let response  = blocking_send(&self.connection, msg)?;
        match response.read4() {
            Ok(data) => Ok(data),
            Err(_) => Err(String::from("dbus::arg::TypeMismatchError (dbus::Message::read4())"))
        }
    }

    // Name, Description, Service, Path
    pub fn position_provider_info(&self) -> Result<(String, String, String, String), String> {
        let interface = "org.freedesktop.Geoclue.MasterClient";
        let method    = "GetPositionProvider";
        let msg       = dbus::Message::new_method_call(DESTINATION,
                                                       &self.client_path,
                                                       interface,
                                                       method)?;
        let response  = blocking_send(&self.connection, msg)?;
        match response.read4() {
            Ok(data) => Ok(data),
            Err(_) => Err(String::from("dbus::arg::TypeMismatchError (dbus::Message::read4())"))
        }
    }

    pub fn provider_name(&self) -> String {
        self.provider_info()
            .unwrap_or((String::new(), String::new())).0
    }

    pub fn addr_provider_name(&self) -> String {
        self.addr_provider_info()
            .unwrap_or((String::new(), String::new(), String::new(), String::new())).0
    }

    pub fn position_provider_name(&self) -> String {
        self.position_provider_info()
            .unwrap_or((String::new(), String::new(), String::new(), String::new())).0
    }

    pub fn provider_status(&self, dest: &str, path: &str) -> Result<i32, String> {
        let interface = "org.freedesktop.Geoclue";
        let method    = "GetStatus";
        let msg       = dbus::Message::new_method_call(dest, path, interface, method)?;
        let response  = blocking_send(&self.connection, msg)?;
        let resp: i32 = match response.read1() {
            Ok(data) => data,
            Err(_)   => {
                return Err(String::from("dbus::arg::TypeMismatchError (dbus::Message::read4())"))
            },
        };
        Ok(resp)
    }
}

fn blocking_send(c: &dbus::Connection, msg: dbus::Message) -> Result<dbus::Message, String> {
    match c.send_with_reply_and_block(msg, TIMEOUT) {
        Ok(r)  => Ok(r),
        Err(e) => { // convert to the same error type as Message::new_method_call could return
            Err(format!("{}: {}", e.name().unwrap_or("NO_NAME"),
                                  e.message().unwrap_or("NO_MESSAGE")))
        },
    }
}

pub fn available_clients(c: &dbus::Connection) -> Vec<usize> {
    fn client_props(c: &dbus::Connection, client: i32) -> String {
        let path      = format!("/org/freedesktop/Geoclue/Master/client{}", client);
        let interface = "org.freedesktop.DBus.Introspectable";
        let method    = "Introspect";
        let msg       = dbus::Message::new_method_call(DESTINATION, path, interface, method).unwrap();
        let response  = blocking_send(&c, msg).unwrap();
        let resp: String = response.read1().unwrap();
        resp
    }

    let mut clients = Vec::new();
    let res: Vec<bool> = (0 .. 255).map(|i| {
        client_props(c, i).len() > 158 // 158 chars for an empty response. Large responses indicate
    }).collect();                      // something is actually available. Yes, this is hacky af

    for (i, avail) in res.iter().enumerate() {
        if *avail { clients.push(i) }
    }
    clients // [0, 1, 2, ...]
}

