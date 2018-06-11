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

        client.set_desktop_id("desktop".into()).unwrap();
        client.start().unwrap();

        GeoClue2 { 
            connection, 
            manager,
            client,
        }
    }

    pub fn add_agent(&self, id: &str) -> Result<(), dbus::Error> {
        return self.manager.add_agent(id)
    }

    pub fn get_in_use(&self) -> Result<bool, dbus::Error> {
        return self.manager.get_in_use()
    }

    pub fn get_available_accuracy_level(&self) -> Result<u32, dbus::Error> {
        return self.manager.get_available_accuracy_level()
    }

    pub fn get_latitude(&self) -> Result<f64, dbus::Error> {
        return self.client.get_latitude()
    }

    pub fn get_longitude(&self) -> Result<f64, dbus::Error> {
        return self.client.get_longitude()
    }
    
    pub fn get_accuracy(&self) -> Result<f64, dbus::Error> {
        return self.client.get_accuracy()
    }
    
    pub fn get_altitude(&self) -> Result<f64, dbus::Error> {
        return self.client.get_altitude()
    }
    
    pub fn get_speed(&self) -> Result<f64, dbus::Error> {
        return self.client.get_speed()
    }
    
    pub fn get_heading(&self) -> Result<f64, dbus::Error> {
        return self.client.get_heading()
    }
}