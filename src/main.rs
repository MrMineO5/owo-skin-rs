mod client;
mod network;
mod muscles;
mod sensations;

use std::net::SocketAddrV4;
use std::str::FromStr;
use muscles::MuscleWithIntensity;
use sensations::MicroSensation;

use crate::client::Client;

fn main() {
    let client = Client::new();

    client.connect(SocketAddrV4::from_str("192.168.68.115:54020").expect("Unable to parse IP address"));

    client.send_sensation(
        MicroSensation::new(
            100,
            10.0,
            30,
            0.0,
            0.0,
            0.0,
            "test".to_string()
        ), 
    MuscleWithIntensity::new(muscles::Muscle::ArmL, 5)
);
    
    client.send_sensation_muscles(
        MicroSensation::new(100,
            10.0,
            30,
            0.0,
            0.0,
            0.0,
            "test".to_string()
        ),
        vec![
            MuscleWithIntensity::new(muscles::Muscle::ArmL, 5),
            MuscleWithIntensity::new(muscles::Muscle::ArmR, 10),
            MuscleWithIntensity::new(muscles::Muscle::AbdominalL, 1),
        ]
);
}