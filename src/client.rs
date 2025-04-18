use crate::network::UDPNetwork;
use crate::sensation::Sensation;
use std::net::ToSocketAddrs;
use std::thread::sleep;
use std::time::Duration;
use crate::auth::GameAuth;

pub struct Client {
    auth: GameAuth,
    network: UDPNetwork,
}
impl Client {
    pub fn new(auth: GameAuth) -> Client {
        let network = UDPNetwork::new();
        Client { auth, network }
    }

    pub fn auto_connect(&self) {
        self.connect(("255.255.255.255", 54020))
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) {
        loop {
            if let Some((data, src)) = self.network.recv_from() {
                if data == "okay" {
                    let message = format!("{}*AUTH*{}", self.auth.id, self.auth.baked_sensations);
                    self.network.send_to(message.as_str(), src);
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

    pub fn send_sensation(&self, sensation: Sensation) {
        self.network
            .send(format!("{}*SENSATION*{}", self.auth.id, sensation.to_packet()).as_str())
    }
}
