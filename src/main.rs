extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate dbus;
use dbus::*;

mod location;
use location::Location;

fn main() {
    let c = Connection::get_private(BusType::Session).unwrap();

    println!("{:#?}", location::provider_info(&c));
    println!("{:#?}", location::addr_provider_info(&c));
    println!("{:#?}", location::position_provider_info(&c));

    let location = Location::get_address(&c).unwrap()
                            .merge(Location::get_position(&c).unwrap())
                            .as_json();

    println!("{}", location.unwrap());

}
