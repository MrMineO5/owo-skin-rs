use owo_skin::auth::GameAuth;
use owo_skin::client::Client;
use owo_skin::muscles;
use owo_skin::muscles::MuscleWithIntensity;
use owo_skin::sensation::Sensation;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let client = Client::new(GameAuth::default());

    client.auto_connect();

    sleep(Duration::from_secs(2));

    client.send_sensation(Sensation::with_muscles(
        Sensation::micro_sensation(100, 1., 20, 0.0, 0.0, 0.0, "test".to_string()),
        muscles::ALL
            .map(|m| MuscleWithIntensity::new(m, 100))
            .to_vec(),
    ));
}
