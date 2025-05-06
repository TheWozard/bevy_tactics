use bevy::prelude::*;

use crate::game::grid;

pub fn plugin(app: &mut App) {
    // app.add_plugins(game::plugin);
}

#[derive(Component)]
#[require(Transform, grid::Location)]
pub struct Unit {}

impl Unit {
    pub fn new() -> Self {
        Unit {}
    }
}
