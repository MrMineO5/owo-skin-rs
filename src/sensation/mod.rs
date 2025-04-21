use crate::muscles::MuscleWithIntensity;
use crate::sensation::baked_sensation::BakedSensation;
use crate::sensation::micro_sensation::MicroSensation;
use crate::sensation::sensation_sequence::SensationSequence;
use crate::sensation::sensation_with_muscles::SensationWithMuscles;

mod micro_sensation;
mod sensation_sequence;
mod sensation_with_muscles;
mod baked_sensation;

pub enum Sensation {
    MicroSensation(Box<MicroSensation>),
    SensationSequence(Box<SensationSequence>),
    SensationWithMuscles(Box<SensationWithMuscles>),
    BakedSensation(Box<BakedSensation>)
}
impl Sensation {

    pub fn micro_sensation(frequency: u8, duration: f32, intensity: u8, ramp_up: f32, ramp_down: f32, exit_delay: f32, name: String) -> Sensation {
        let frequency = frequency.clamp( 1, 100);
        let duration = duration.clamp(0.1, 20.0);
        let intensity = intensity.clamp(0, 100);
        let ramp_up = ramp_up.clamp(0.0, 2.0);
        let ramp_down = ramp_down.clamp(0.0, 2.0);
        let exit_delay = exit_delay.clamp(0.0, 20.0);
        let name = name;

        Sensation::MicroSensation(Box::new(MicroSensation { frequency, duration, intensity, ramp_up, ramp_down, exit_delay, name }))
    }

    pub fn sequence(sensations: Vec<Sensation>) -> Sensation {
        Sensation::SensationSequence(Box::new(SensationSequence { sensations }))
    }

    pub fn with_muscles(sensation: Sensation, muscles: Vec<MuscleWithIntensity>) -> Sensation {
        Sensation::SensationWithMuscles(Box::new(SensationWithMuscles { sensation, muscles }))
    }

    pub fn baked(id: u32) -> Sensation {
        Sensation::BakedSensation(Box::new(BakedSensation { id }))
    }


    pub fn to_packet(&self) -> String {
        match self {
            Sensation::MicroSensation(micro_sensation) => {
                micro_sensation.to_packet()
            }
            Sensation::SensationSequence(sensation_sequence) => {
                sensation_sequence.to_packet()
            }
            Sensation::SensationWithMuscles(sensation_with_muscles) => {
                sensation_with_muscles.to_packet()
            }
            Sensation::BakedSensation(baked_sensation) => {
                baked_sensation.to_packet()
            }
        }
    }
}
