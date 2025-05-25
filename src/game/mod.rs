use bevy::prelude::*;

mod game;
mod gizmo;
mod grid;
mod unit;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins(game::plugin);
    app.add_plugins(gizmo::plugin);
    app.add_plugins(grid::plugin);
    app.add_plugins(unit::plugin);

    app.add_systems(Startup, init);
}

fn init(mut commands: Commands) {
    let mut entity = commands.spawn_empty();
    let size = IVec2::new(10, 10);
    let grid = grid::Grid::new(size);

    entity.with_related::<grid::GridOwner>((
        unit::Unit {
            unit_group: unit::UnitGroup::Player,
            unit_type: unit::UnitType::Offensive,
        },
        grid.get_location(IVec2::new(0, 0)).unwrap(),
    ));
    entity.with_related::<grid::GridOwner>((
        unit::Unit {
            unit_group: unit::UnitGroup::Enemy,
            unit_type: unit::UnitType::Defensive,
        },
        grid.get_location(size - 1).unwrap(),
    ));
    entity.with_related::<grid::GridOwner>((
        unit::Unit {
            unit_group: unit::UnitGroup::Neutral,
            unit_type: unit::UnitType::Mixed,
        },
        grid.get_location(size - 2).unwrap(),
    ));

    entity.insert((grid, game::TurnOrder::default()));
}
