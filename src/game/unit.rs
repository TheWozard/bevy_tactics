use bevy::prelude::*;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.register_type::<Unit>();
    app.register_type::<UnitGroup>();
    app.register_type::<UnitType>();
    app.register_type::<Stats>();
}

#[derive(Clone, Debug, Reflect)]
pub enum UnitGroup {
    Player,
    Enemy,
    Neutral,
}

#[derive(Clone, Debug, Reflect)]
pub enum UnitType {
    Offensive,
    Defensive,
    Mixed,
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform, Stats, Name::new("Unit"))]
pub struct Unit {
    pub unit_group: UnitGroup,
    pub unit_type: UnitType,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Stats {
    pub movement: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats { movement: 1 }
    }
}
