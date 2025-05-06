use bevy::prelude::*;

mod grid;
mod unit;

pub fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, unit::plugin));
}
