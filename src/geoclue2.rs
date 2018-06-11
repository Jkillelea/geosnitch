#![allow(dead_code)]

extern crate dbus;
use dbus::*;
use geoclue2_sys::*;

use std::rc::Rc;

const DESTINATION: &'static str = "org.freedesktop.GeoClue2";
const PATH: &'static str = "/org/freedesktop/GeoClue2/Manager";

pub struct GeoClue2<'a> {
    connection: Rc<Connection>,
    manager: ConnPath<'a, Rc<Connection>>,
    client: ConnPath<'a, Rc<Connection>>,
}

impl<'a> GeoClue2<'a> {
    pub fn new() -> GeoClue2<'a> {
        let connection = Rc::new(Connection::get_private(BusType::System).unwrap());   

        let busname = BusName::new(DESTINATION).unwrap();
        let path = Path::new(PATH).unwrap();
        let timeout = 2000i32;

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
        
        println!("{:#?}", client);

        GeoClue2 { 
            connection, 
            manager,
            client,
        }
    }
}