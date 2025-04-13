# OWO-Skin-rs
A minimal implementation of the [OWO Skin Application Protocol](https://owogame.com/) written in Rust.

## Features
- Connect to the OWO application
- Auto connect using UDP broadcast
- Send microsensations to the OWO application

## Usage
```rust
use owo_skin::client::Client;
use owo_skin::muscles::{Muscle, MuscleWithIntensity};
use owo_skin::sensations::MicroSensation;

fn main() {
    let client = Client::new();

    client.auto_connect();

    sleep(Duration::from_secs(2));

    client.send_sensation_muscles(
        MicroSensation::new(100,
                            1.,
                            20,
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
```