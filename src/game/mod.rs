use bevy::prelude::*;

mod drag_n_drop;
mod game;
mod gizmo;
mod grid;
mod tiles;
mod unit;

#[allow(dead_code)]
mod utils;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins(game::plugin);
    app.add_plugins(gizmo::plugin);
    app.add_plugins(grid::plugin);
    app.add_plugins(unit::plugin);
    app.add_plugins(drag_n_drop::plugin);

    app.add_systems(Startup, (init, tiles::populate).chain());
}

fn init(mut commands: Commands) {
    let root = commands.spawn_empty().id();
    let size = IVec2::new(10, 10);
    let mut grid = grid::Grid::new(size);

    let steps = 2;
    let units = 10;

    let spawn_space = IVec2::new(size.x, size.y / 2);
    let mut team_1_spaces = utils::Grid::new(spawn_space);
    for i in 0..units {
        if let Some(loc) = team_1_spaces.random(()) {
            let enemy = grid.spawn(
                &mut commands,
                &loc,
                (
                    unit::Unit {
                        team: 1,
                        color: Color::linear_rgb(1.0, 0.0, 0.0),
                        sides: 3 + i,
                    },
                    unit::Movement::new(steps),
                    unit::Health::new(3),
                    unit::Speed::new(rand::random_range(2..10)),
                ),
            );
            commands
                .entity(root)
                .add_one_related::<grid::GridOwner>(enemy.unwrap());
        }
    }

    let mut team_2_spaces = utils::Grid::new(spawn_space);
    for i in 0..units {
        if let Some(loc) = team_2_spaces.random(()) {
            let enemy = grid.spawn(
                &mut commands,
                &(size - 1 - loc),
                (
                    unit::Unit {
                        team: 2,
                        color: Color::linear_rgb(0.0, 0.0, 1.0),
                        sides: 3 + i,
                    },
                    unit::Movement::new(steps),
                    unit::Health::new(3),
                    unit::Speed::new(rand::random_range(2..10)),
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
