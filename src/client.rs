use crate::muscles::MuscleWithIntensity;
use crate::network::UDPNetwork;
use crate::sensations::MicroSensation;
use std::net::ToSocketAddrs;
use std::thread::sleep;
use std::time::Duration;

pub struct Client {
    pub network: UDPNetwork,
}
impl Client {
    pub fn new() -> Client {
        let network = UDPNetwork::new();
        Client { network }
    }

    pub fn auto_connect(&self) {
        self.connect(("255.255.255.255", 54020))
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) {
        loop {
            if let Some((data, src)) = self.network.recv_from() {
                if data == "okay" {
                    self.network.send_to("0*AUTH*", src);
                } else if data == "pong" {
                    self.network.connect(src);
                    break;
                }
            } else {
                self.network.send_to("ping", &addr);
            }

            sleep(Duration::from_secs(3));
        }
    }

    pub fn send_sensation(&self, micro_sensation: MicroSensation, muscle: MuscleWithIntensity) {
        self.network.send(
            format!(
                "0*SENSATION*{}|{}",
                micro_sensation.to_packet(),
                muscle.to_packet()
            )
            .as_str(),
        )
    }

    pub fn send_sensation_muscles(
        &self,
        micro_sensation: MicroSensation,
        muscles: Vec<MuscleWithIntensity>,
    ) {
        let tmp = muscles
            .into_iter()
            .map(|x| x.to_packet())
            .collect::<Vec<String>>()
            .join(",");
        self.network
            .send(format!("0*SENSATION*{}|{}", micro_sensation.to_packet(), tmp).as_str())
    }
}
