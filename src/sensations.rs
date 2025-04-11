pub struct MicroSensation {
    frequency: u8,
    duration: f32,
    intensity: u8,
    ramp_up: f32,
    ramp_down: f32,
    exit_delay: f32,
    name: String,
}

impl MicroSensation {
    pub fn new(frequency: u8, duration: f32, intensity: u8, ramp_up: f32, ramp_down: f32, exit_delay: f32, name: String) -> MicroSensation {
        let frequency = frequency.clamp( 1, 100);
		let duration = duration.clamp(0.1, 20.0).round();
		let intensity = intensity.clamp(0, 100);
		let ramp_up = ramp_up.clamp(0.0, 2.0).round();
		let ramp_down = ramp_down.clamp(0.0, 2.0).round();
		let exit_delay = exit_delay.clamp(0.0, 20.0).round();
		let name = name;

        MicroSensation { frequency, duration, intensity, ramp_up, ramp_down, exit_delay, name }
    }

    pub fn to_packet(&self) -> String {
        format!("{},{},{},{},{},{},{}",
            self.frequency,
            (self.duration * 10.0).round() as u32,
            self.intensity,
            (self.ramp_up * 1000.0).round() as u32,
            (self.ramp_down * 1000.0).round() as u32,
            (self.exit_delay * 10.0).round() as u32,
            self.name,
        )
    }
}