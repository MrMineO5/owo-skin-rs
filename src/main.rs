mod client;
mod network;
mod muscles;
mod sensations;

use std::net::SocketAddrV4;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use muscles::MuscleWithIntensity;
use sensations::MicroSensation;

use crate::client::Client;

fn main() {
    let client = Client::new();

    // client.connect(SocketAddrV4::from_str("192.168.68.115:54020").expect("Unable to parse IP address"));
    client.auto_connect();

    sleep(Duration::from_secs(2));

    client.send_sensation_muscles(
        MicroSensation::new(100,
            1.,
            100,
            0.0,
            0.0,
            0.0,
            "test".to_string()
        ),
        vec![
            MuscleWithIntensity::new(muscles::Muscle::DorsalL, 100),
        ]
    );
}