use specs::VecStorage;
use specs::Component;

pub enum Team {
    Player,
    Enemy,
}

/// Indicates the team this entity is on
pub struct TeamComponent {
    pub team: Team,
}

impl Component for TeamComponent {
    type Storage = VecStorage<Self>;
}