// Message rendering

#[derive(Clone, Copy, Debug)]
pub enum Perspective {
    FirstPerson,   // You attack
    SecondPerson,  // X attacks you
    ThirdPerson,   // X attacks Y
}

pub struct CombatMessage {
    pub attacker_name: String,
    pub defender_name: String,
    pub action: String,
    pub damage: u16,
}
