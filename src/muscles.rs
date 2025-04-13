#[repr(u8)]
pub enum Muscle {
    PectoralR = 0,
    PectoralL = 1,
    AbdominalR = 2,
    AbdominalL = 3,
    ArmR = 4,
    ArmL = 5,
    DorsalR = 6,
    DorsalL = 7,
    LumbarR = 8,
    LumbarL = 9,
}

pub const FRONT: [Muscle; 6] = [Muscle::PectoralR, Muscle::PectoralL, Muscle::AbdominalR, Muscle::AbdominalL, Muscle::ArmR, Muscle::ArmL];
pub const BACK: [Muscle; 4] = [Muscle::DorsalR, Muscle::DorsalL, Muscle::LumbarR, Muscle::LumbarL];
pub const ALL: [Muscle; 10] = [Muscle::PectoralR, Muscle::PectoralL, Muscle::AbdominalR, Muscle::AbdominalL, Muscle::ArmR, Muscle::ArmL, Muscle::DorsalR, Muscle::DorsalL, Muscle::LumbarR, Muscle::LumbarL];


pub struct MuscleWithIntensity {
    muscle: Muscle, 
    intensity: u8
}

impl MuscleWithIntensity {
    pub fn new(muscle: Muscle, intensity: u8) -> MuscleWithIntensity {
        MuscleWithIntensity { muscle, intensity }
    }
    pub fn to_packet(self) -> String {
        format!("{}%{}", self.muscle as u8, self.intensity)
    }
}