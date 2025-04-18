use crate::muscles::MuscleWithIntensity;
use crate::sensation::Sensation;

pub struct SensationWithMuscles {
    pub(crate) sensation: Box<Sensation>,
    pub(crate) muscles: Vec<MuscleWithIntensity>,
}
impl SensationWithMuscles {
    pub fn to_packet(&self) -> String {
        let tmp = self
            .muscles
            .iter()
            .map(|x| x.to_packet())
            .collect::<Vec<String>>()
            .join(",");
        format!("{}|{}", self.sensation.to_packet(), tmp)
    }
}
