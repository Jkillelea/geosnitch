use std::fs::File;
use std::convert;
use std::path::Path;
use std::io::prelude::*;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub server: String,           // server name:port
    pub username: Option<String>, // login username
    pub password: Option<String>, // login password
    pub client_id: String,        // MQTT client ID
    pub topic: String,            // MQTT topic
    pub send_period: Option<u64>, // send every $(send_period) seconds
}

/* 'path' should point to a file that looks like:
{
  "server":"server:port",
  "username":"account_username", (optional)
  "password":"account_password", (optional)
  "client_id":"client_id",
  "topic":"publish_topic"
}
*/
pub fn get<P: convert::AsRef<Path>>(path: P) -> MqttConfig {
    let mut conf = String::new(); // read in config
    let mut conf_file = File::open(path).unwrap();
    conf_file.read_to_string(&mut conf).unwrap();
    let conf: MqttConfig = serde_json::from_str(&conf).unwrap();
    conf
}
