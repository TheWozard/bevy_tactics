use bevy::prelude::*;

mod animate;
mod game;
mod gizmo;
mod grid;
mod tiles;
mod unit;
mod utils;

use crate::theme::Sprites;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins(game::plugin);
    app.add_plugins(gizmo::plugin);
    app.add_plugins(grid::plugin);
    app.add_plugins(unit::plugin);
    app.add_plugins(animate::plugin);

    app.add_systems(Startup, (init, tiles::populate).chain());
}

fn init(mut commands: Commands, sprites: Res<Sprites>) {
    let root = commands.spawn_empty().id();
    let size = IVec2::new(40, 40);
    let mut grid = grid::Grid::new(size);

    let step_range = 2..4;
    let units = 100;

    let spawn_space = IVec2::new(size.x, size.y / 3);
    let team_1_spaces = utils::SquareSelection::new(IVec2::ZERO, spawn_space);
    for _ in 0..units {
        if let Some(index) = team_1_spaces.random_open(&grid.grid) {
            let enemy = grid.spawn(
                &mut commands,
                &grid.grid.location(index),
                (
                    Transform::from_translation(
                        grid.grid.index_to_vec2(index, sprites.scale).extend(1.0),
                    ),
                    Sprite {
                        color: Color::linear_rgb(1.0, 0.0, 0.0),
                        ..sprites.unit_sprite()
                    },
                    unit::Unit { team: 1 },
                    unit::Movement::new(rand::random_range(step_range.clone())),
                    unit::Health::new(3),
                    unit::Speed::new(1),
                ),
            );
            commands
                .entity(root)
                .add_one_related::<grid::GridOwner>(enemy.unwrap());
        }
    }

    let team_2_spaces = utils::SquareSelection::new(
        IVec2::new(0, grid.grid.size().y - spawn_space.y),
        grid.grid.size(),
    );
    for _ in 0..units {
        if let Some(index) = team_2_spaces.random_open(&grid.grid) {
            let enemy = grid.spawn(
                &mut commands,
                &grid.grid.location(index),
                (
                    Transform::from_translation(
                        grid.grid.index_to_vec2(index, sprites.scale).extend(1.0),
                    ),
                    Sprite {
                        color: Color::linear_rgb(0.0, 0.0, 1.0),
                        ..sprites.unit_sprite()
                    },
                    unit::Unit { team: 2 },
                    unit::Movement::new(rand::random_range(step_range.clone())),
                    unit::Health::new(3),
                    unit::Speed::new(2),
                ),
            );
            commands
                .entity(root)
                .add_one_related::<grid::GridOwner>(enemy.unwrap());
        }
    }

    commands
        .entity(root)
        .insert((grid, game::TurnOrder::default()));
}
