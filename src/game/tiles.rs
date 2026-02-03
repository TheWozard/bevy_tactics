use bevy::prelude::*;

use super::grid;
use crate::theme::Textures;
use crate::util::cords;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_observer(populate_grid);
}

pub fn populate_grid(
    trigger: On<Add, grid::Grid>,
    mut commands: Commands,
    textures: Res<Textures>,
    mut query: Query<(&mut grid::Grid, &grid::GridScale, Entity)>,
) {
    let entity = trigger.event_target();
    if let Ok((mut grid, scale, entity)) = query.get_mut(entity) {
        for index in 0..grid.spaces() {
            let location = cords::index_to_location(&grid.size(), index as usize);
            grid.spawn(
                &mut commands,
                &grid::EntityKind::Tile,
                &location,
                entity,
                (
                    Tile {},
                    Sprite {
                        ..textures.tile.sprite()
                    },
                    Transform::from_translation(cords::location_to_translation(
                        &location,
                        scale.scale(),
                        -1,
                    )),
                    Name::new("Tile"),
                ),
            );
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Tile {}
