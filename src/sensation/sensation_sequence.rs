use crate::sensation::Sensation;

pub struct SensationSequence {
    pub(crate) sensations: Vec<Sensation>
}
impl SensationSequence {
    pub fn to_packet(&self) -> String {
        self.sensations
            .iter()
            .map(|x| x.to_packet())
            .collect::<Vec<String>>()
            .join("&")
    }
}
