pub struct GameAuth {
    pub(crate) id: u64,
    pub(crate) baked_sensations: String,
}
impl GameAuth {
    pub fn default() -> GameAuth {
        GameAuth {
            id: 0,
            baked_sensations: String::new(),
        }
    }
    pub fn new(id: u64, baked_sensations: String) -> GameAuth {
        GameAuth {
            id,
            baked_sensations,
        }
    }

    pub fn to_packet(&self) -> String {
        format!("{}|{}", self.id, self.baked_sensations)
    }
}
