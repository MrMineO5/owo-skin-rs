pub struct BakedSensation {
    pub(crate) id: u32,
}
impl BakedSensation {
    pub fn to_packet(&self) -> String {
        format!("{}", self.id)
    }
}
