extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate dbus;
use dbus::*;

mod location;
use location::Location;

fn main() {
    let c = Connection::get_private(BusType::Session).unwrap();

    let location = Location::get_address(&c).unwrap()
                            .merge(Location::get_position(&c).unwrap())
                            .as_json();
                            
    println!("{}", location.unwrap());
}
