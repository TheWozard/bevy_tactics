use bevy::{ecs::query, prelude::*};

use super::grid;
use crate::theme::Textures;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_observer(populate_grid);
}

pub fn populate_grid(trigger: On<Add, grid::Grid>, mut commands: Commands, textures: Res<Textures>, query: Query<(&mut grid::Grid, &mut grid::GridScale)>) {
    let entity = trigger.event_target();
    if let Ok((grid, grid_scale)) = query.get(entity) {
        let scale = textures.tile.scale();
        grid_scale.scale = scale;
        for transform in grid
            .grid
            .iter_to_transform(grid.grid.iter_entire_grid(), textures.scale)
        {
            commands.spawn((
                Tile {},
                Sprite {
                    ..textures.tile_sprite()
                },
                Transform::from_translation(transform),
                Name::new("Tile"),
            ));
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct GridScale {
    pub scale: Vec2,
}

impl GridScale {
    pub fn new(scale: Vec2) -> Self {
        GridScale { scale }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Tile {}
