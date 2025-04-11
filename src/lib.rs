mod client;
mod network;
mod muscles;
mod sensations;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddrV4;
    use std::str::FromStr;
    use crate::client::Client;

    #[test]
    fn it_works() {
        let client = Client::new();

        client.network.connect(SocketAddrV4::from_str("192.168.68.115:54020").expect("Unable to parse IP address"));

        client.find_server();
    }
}
