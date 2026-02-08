use crate::network::UDPNetwork;
use crate::sensation::Sensation;
use std::net::ToSocketAddrs;
use std::thread::sleep;
use std::time::Duration;
use crate::auth::GameAuth;

const OWO_PORT: u16 = 54020;
const AUTO_CONNECT_ADDRESS: (&str, u16) = ("255.255.255.255", OWO_PORT);

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
        self.connect(AUTO_CONNECT_ADDRESS)
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) {
        loop {
            if self.connect_non_blocking(&addr) {
                break;
            }
        }
    }

    pub fn auto_connect_non_blocking(&self) -> bool {
        self.connect_non_blocking(&AUTO_CONNECT_ADDRESS)
    }
    pub fn connect_non_blocking<A: ToSocketAddrs>(&self, addr: &A) -> bool {
        self.network.send_to("ping", &addr);

        sleep(Duration::from_secs(3));

        if let Some((data, src)) = self.network.recv_from() {
            if data == "okay" {
                let message = format!("{}*AUTH*{}", self.auth.id, self.auth.baked_sensations);
                self.network.send_to(message.as_str(), src);
            } else if data == "pong" {
                self.network.connect(src);
                return true;
            }
        }
        false
    }

    pub fn send_sensation(&self, sensation: Sensation) {
        self.network
            .send(format!("{}*SENSATION*{}", self.auth.id, sensation.to_packet()).as_str())
    }
}
