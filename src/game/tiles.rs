use bevy::prelude::*;

use super::grid;
use crate::theme::Sprites;

pub fn plugin(app: &mut bevy::prelude::App) {
    // app.add_systems(Update, color);
}

pub fn populate(mut commands: Commands, grid_query: Query<&mut grid::Grid>, sprites: Res<Sprites>) {
    if let Ok(grid) = grid_query.single() {
        for transform in grid
            .grid
            .iter_to_transform(grid.grid.iter_entire_grid(), sprites.scale)
        {
            commands.spawn((
                Tile {},
                Sprite {
                    // color: Color::hsl(120.0, 0.3, 0.5),
                    ..sprites.tile_sprite()
                },
                Transform::from_translation(transform),
                Name::new("Tile"),
            ));
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Tile {}

// pub fn color(mut query: Query<&mut Sprite, With<Tile>>, time: Res<Time>) {
//     for mut sprite in query.iter_mut() {
//         let mut hue = sprite.color.hue() + time.delta_secs() * 10.0;
//         if hue >= 360.0 {
//             hue -= 360.0;
//         }
//         sprite.color.set_hue(hue);
//     }
// }
