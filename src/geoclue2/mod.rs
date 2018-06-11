#![allow(dead_code)]

extern crate dbus;
use dbus::*;

// auto generated bindings
mod manager_sys;
use self::manager_sys::*;
mod client_sys;
use self::client_sys::*;
mod client_location_sys;
use self::client_location_sys::*;

use std::rc::Rc;


const DESTINATION: &'static str = "org.freedesktop.GeoClue2";
const PATH: &'static str = "/org/freedesktop/GeoClue2/Manager";

#[derive(Debug)]
pub struct GeoClue2<'a> {
    connection: Rc<Connection>,
    manager: ConnPath<'a, Rc<Connection>>,
    client: ConnPath<'a, Rc<Connection>>,
}

impl<'a> GeoClue2<'a> {
    pub fn new() -> GeoClue2<'a> {
        let timeout = 2000i32;

        let connection = Rc::new(
                Connection::get_private(BusType::System).unwrap()
            );   

        let manager = ConnPath {
                conn: connection.clone(),
                dest: BusName::new(DESTINATION).unwrap(),
                path: Path::new(PATH).unwrap(),
                timeout
            };
        
        let client = ConnPath {
                conn: connection.clone(),
                dest: BusName::new(DESTINATION).unwrap(),
                path: manager.get_client().unwrap(),
                timeout
            };

        GeoClue2 { 
            connection, 
            manager,
            client,
        }
    }
}