#![allow(dead_code)]
use std::collections::HashMap;
use serde_json;
use dbus;

const TIMEOUT: i32 = 10; // miliseconds?
const DESTINATION: &'static str = "org.freedesktop.Geoclue.Master";
const PATH: &'static str = "/org/freedesktop/Geoclue/Master/client2";
// client0, 1, 2, or 3 on my computer. client0 doesn't seem to have the ability to get
// position information, only address info. Aside from that, they all seem identical

#[derive(Debug, Serialize)]
pub struct Location {            // Wrapping every field in Option makes it more flexible.
    country:     Option<String>, // Can encode it as JSON even if some fields are
    postalcode:  Option<String>, // missing
    region:      Option<String>,
    timezone:    Option<String>,
    locality:    Option<String>,
    countrycode: Option<String>,
    lat_lon:     Option<(f64, f64)>,
}

impl Location {
    // Query the D-Bus for address
    pub fn get_address(c: &dbus::Connection) -> Result<Self, String> {
        let interface = "org.freedesktop.Geoclue.Address";
        let method    = "GetAddress";
        let msg       = dbus::Message::new_method_call(DESTINATION, PATH, interface, method)?;
        let response  = blocking_send(c, msg)?;
        // awkward argument parsing
        let (_time, mut location): (i32, HashMap<String, String>) = response.read2()
                                                                       .unwrap_or((0, HashMap::new()));
        Ok(Location { // take values out from the HashMap
            country:     location.remove("country"), // Option<String>
            postalcode:  location.remove("postalcode"),
            region:      location.remove("region"),
            timezone:    location.remove("timezone"),
            locality:    location.remove("locality"),
            countrycode: location.remove("countrycode"),
            lat_lon: None,
        })
    }

    // Query the D-Bus for position
    pub fn get_position(c: &dbus::Connection) -> Result<Self, String> {
        let interface = "org.freedesktop.Geoclue.Position";
        let method    = "GetPosition";
        let msg       = dbus::Message::new_method_call(DESTINATION, PATH, interface, method)?;
        let response  = blocking_send(c, msg)?;
        let (_n, _t, lat, lon): (i32, i32, f64, f64) = response.read4().unwrap_or((0, 0, 0.0, 0.0));
        Ok(Location { // take values from the HashMap
            country:     None, // Option<String>
            postalcode:  None,
            region:      None,
            timezone:    None,
            locality:    None,
            countrycode: None,
            lat_lon: Some((lat, lon)),
        })

    }

    pub fn empty() -> Self {
        Location {
            country:     None,
            postalcode:  None,
            region:      None,
            timezone:    None,
            locality:    None,
            countrycode: None,
            lat_lon:     None,
        }
    }

    // have serde produce a JSON string
    pub fn as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    // if the other struct contains a field, overwrite it in this struct
    pub fn merge(mut self, other: Self) -> Self {
        if other.country.is_some() {
            self.country = other.country;
        }
        if other.postalcode.is_some() {
            self.postalcode = other.postalcode;
        }
        if other.region.is_some() {
            self.region = other.region;
        }
        if other.timezone.is_some() {
            self.timezone = other.timezone;
        }
        if other.locality.is_some() {
            self.locality = other.locality;
        }
        if other.countrycode.is_some() {
            self.countrycode = other.countrycode;
        }
        if other.lat_lon.is_some() {
            self.lat_lon = other.lat_lon;
        }
        self
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

// Some useful public functions
// Name, Description
pub fn provider_info(c: &dbus::Connection) -> Result<(String, String), String> {
    let interface = "org.freedesktop.Geoclue";
    let method    = "GetProviderInfo";
    let msg       = dbus::Message::new_method_call(DESTINATION, PATH, interface, method)?;
    let response  = blocking_send(c, msg)?;
    match response.read2() {
        Ok(data) => Ok(data),
        Err(_) => Err(String::from("dbus::arg::TypeMismatchError (dbus::Message::read2())"))
    }
}

// Name, Description, Service, Path
pub fn addr_provider_info(c: &dbus::Connection) -> Result<(String, String, String, String), String> {
    let interface = "org.freedesktop.Geoclue.MasterClient";
    let method    = "GetAddressProvider";
    let msg       = dbus::Message::new_method_call(DESTINATION, PATH, interface, method)?;
    let response  = blocking_send(c, msg)?;
    match response.read4() {
        Ok(data) => Ok(data),
        Err(_) => Err(String::from("dbus::arg::TypeMismatchError (dbus::Message::read4())"))
    }
}

// Name, Description, Service, Path
pub fn position_provider_info(c: &dbus::Connection) -> Result<(String, String, String, String), String> {
    let interface = "org.freedesktop.Geoclue.MasterClient";
    let method    = "GetPositionProvider";
    let msg       = dbus::Message::new_method_call(DESTINATION, PATH, interface, method)?;
    let response  = blocking_send(c, msg)?;
    match response.read4() {
        Ok(data) => Ok(data),
        Err(_) => Err(String::from("dbus::arg::TypeMismatchError (dbus::Message::read4())"))
    }
}

pub fn provider_name(c: &dbus::Connection) -> String {
    provider_info(c).unwrap_or((String::new(), String::new())).0
}
pub fn addr_provider_name(c: &dbus::Connection) -> String {
    addr_provider_info(c).unwrap_or((String::new(), String::new(), String::new(), String::new())).0
}
pub fn position_provider_name(c: &dbus::Connection) -> String {
    position_provider_info(c).unwrap_or((String::new(), String::new(), String::new(), String::new())).0
}
