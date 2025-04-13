use owo_skin::client::Client;
use owo_skin::muscles;
use owo_skin::muscles::MuscleWithIntensity;
use owo_skin::sensations::MicroSensation;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let client = Client::new();

    client.auto_connect();

    sleep(Duration::from_secs(2));

    client.send_sensation_muscles(
        MicroSensation::new(100, 1., 20, 0.0, 0.0, 0.0, "test".to_string()),
        muscles::ALL
            .map(|m| MuscleWithIntensity::new(m, 100))
            .to_vec(),
    );
}
