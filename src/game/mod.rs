use bevy::prelude::*;

mod animate;
mod background;
mod camera;
mod effect;
mod game;
mod gizmo;
mod grid;
mod tiles;
mod unit;

use crate::random::RandomSource;
use crate::theme::Textures;
use crate::util::cords;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins(animate::plugin);
    app.add_plugins(background::plugin);
    app.add_plugins(camera::plugin);
    app.add_plugins(effect::plugin);
    app.add_plugins(game::plugin);
    app.add_plugins(gizmo::plugin);
    app.add_plugins(grid::plugin);
    app.add_plugins(tiles::plugin);
    app.add_plugins(unit::plugin);

    app.add_systems(Startup, init);
}

fn init(mut commands: Commands, sprites: Res<Textures>, mut rand: ResMut<RandomSource>) {
    let root = commands.spawn_empty().id();
    let size = IVec2::new(40, 40);
    let mut grid = grid::Grid::new(size);
    let scale = grid::GridScale::new(sprites.tile.scale().as_ivec2());
    let mut turns = game::TurnOrder::default();

    let step_range = 2..4;

    let spawn_space = IVec2::new(size.x, size.y / 3);
    let team_1_spaces = grid::selection::Shape::Square(IVec2::ZERO, spawn_space);
    for _ in 0..4 {
        if let Some(location) = grid.nearest_empty(
            &grid::EntityKind::Unit,
            &team_1_spaces.random(rand.as_mut()),
            &IVec2::ZERO,
            team_1_spaces.clone(),
        ) {
            turns.add_entity_optional(
                grid.spawn(
                    &mut commands,
                    &grid::EntityKind::Unit,
                    &location,
                    root,
                    (
                        Sprite {
                            color: Color::linear_rgb(1.0, 0.0, 0.0),
                            ..sprites.unit.sprite()
                        },
                        Transform::from_translation(cords::location_to_translation(
                            &location,
                            scale.scale(),
                            1,
                        )),
                        unit::Unit { team: 1 },
                        unit::Movement::new(rand::random_range(step_range.clone())),
                        unit::Health::new(50),
                        unit::Attacks::new(3, 10),
                    ),
                ),
                0,
            );
        }
    }

    let team_2_spaces = grid::selection::Shape::Square(IVec2::new(0, size.y - spawn_space.y), size);
    for _ in 0..100 {
        if let Some(location) = grid.nearest_empty(
            &grid::EntityKind::Unit,
            &team_2_spaces.random(rand.as_mut()),
            &IVec2::ZERO,
            team_2_spaces.clone(),
        ) {
            turns.add_entity_optional(
                grid.spawn(
                    &mut commands,
                    &grid::EntityKind::Unit,
                    &location,
                    root,
                    (
                        Sprite {
                            color: Color::linear_rgb(0.0, 0.0, 1.0),
                            ..sprites.unit.sprite()
                        },
                        Transform::from_translation(cords::location_to_translation(
                            &location,
                            scale.scale(),
                            1,
                        )),
                        unit::Unit { team: 2 },
                        unit::Movement::new(rand::random_range(step_range.clone())),
                        unit::Health::new(3),
                        unit::Attacks::new(1, 1),
                    ),
                ),
                1,
            );
        }
    }

    commands.entity(root).insert((grid, scale, turns));
}
