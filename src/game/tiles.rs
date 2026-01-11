use bevy::prelude::*;

use super::grid;
use crate::theme::Sprites;

pub fn populate(mut commands: Commands, grid_query: Query<&mut grid::Grid>, sprites: Res<Sprites>) {
    if let Ok(grid) = grid_query.single() {
        for transform in grid
            .grid
            .iter_to_transform(grid.grid.iter_entire_grid(), sprites.scale)
        {
            commands.spawn((
                Sprite {
                    color: Color::hsl(120.0, 0.3, 0.5),
                    ..sprites.tile_sprite()
                },
                Transform::from_translation(transform),
                Name::new("Tile"),
            ));
        }
    }
}
