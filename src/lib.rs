mod network;
mod muscles;
mod sensations;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::net::UdpSocket;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::network::UDPNetwork;
    use super::*;

    #[test]
    fn it_works() {
        let network = UDPNetwork::new();

        loop {
            if let Some((data, src)) = network.recv_from() {
                println!("Received: {} from {}", data, src);
            } else {
                network.send("ping")
            }

            sleep(Duration::from_secs(3));
        }

    }
}
