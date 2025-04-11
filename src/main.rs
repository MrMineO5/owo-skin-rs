mod client;
mod network;

use std::net::SocketAddrV4;
use std::str::FromStr;
use crate::client::Client;

fn main() {
    let client = Client::new();

    client.connect(SocketAddrV4::from_str("192.168.68.115:54020").expect("Unable to parse IP address"));

    client.send_sensation();
}