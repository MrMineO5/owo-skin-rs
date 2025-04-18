pub struct MicroSensation {
    pub(crate) frequency: u8,
    pub(crate) duration: f32,
    pub(crate) intensity: u8,
    pub(crate) ramp_up: f32,
    pub(crate) ramp_down: f32,
    pub(crate) exit_delay: f32,
    pub(crate) name: String,
}
impl MicroSensation {
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