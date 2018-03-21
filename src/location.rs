#![allow(dead_code)]
use std::collections::HashMap;
use serde_json;
use dbus;

const TIMEOUT: i32 = 20; // miliseconds?
const DESTINATION: &'static str = "org.freedesktop.Geoclue.Master";
const PATH: &'static str = "/org/freedesktop/Geoclue/Master/client2";
// client0, 1, 2, or 3 on my computer. client0 doesn't seem to have the ability to get
// position information, only address info. Aside from that, they all seem identical

#[derive(Debug)]
pub struct LocationSession {

}

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

    pub fn from_location_hashmap(mut location: HashMap<&str, String>) -> Self {
        Location {
            country:     location.remove("country"), // Option<String>
            postalcode:  location.remove("postalcode"),
            region:      location.remove("region"),
            timezone:    location.remove("timezone"),
            locality:    location.remove("locality"),
            countrycode: location.remove("countrycode"),
            lat_lon: None,
        }
    }

    pub fn from_lat_lon(lat: f64, lon: f64) -> Self {
        Location { // take values from the HashMap
            country:     None, // Option<String>
            postalcode:  None,
            region:      None,
            timezone:    None,
            locality:    None,
            countrycode: None,
            lat_lon: Some((lat, lon)),
        }
    }
}

