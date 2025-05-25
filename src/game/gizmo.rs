use bevy::prelude::*;

use super::game;
use super::grid;
use super::unit;

const GRID_SCALE: Vec2 = Vec2::new(40., 40.);
const UNIT_SCALE: f32 = 30.;
const GRID_COLOR: Color = Color::srgb(0.05, 0.05, 0.05);

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Update, (grid_gizmo, units_gizmo, turn_order_gizmo));
}

fn grid_gizmo(mut gizmos: Gizmos, mut query: Query<(&Transform, &grid::Grid)>) {
    for (transform, grid) in query.iter() {
        gizmos
            .grid_2d(
                Isometry2d::from_translation(transform.translation.truncate()),
                grid.get_size().as_uvec2(),
                GRID_SCALE,
                GRID_COLOR,
            )
            .outer_edges();
    }
}

fn grid_location(
    transform: &Transform,
    grid: &grid::Grid,
    grid_location: impl Into<IVec2>,
) -> Isometry2d {
    let offset =
        (grid.get_size().as_vec2() - 1.0) * 0.5 * GRID_SCALE + transform.translation.truncate();
    Isometry2d::from_translation(grid_location.into().as_vec2() * GRID_SCALE - offset)
}

fn units_gizmo(
    mut gizmos: Gizmos,
    unit_query: Query<(&unit::Unit, &grid::GridLocation)>,
    grid_query: Query<(&Transform, &grid::Grid, &grid::GridOwned)>,
) {
    for (transform, grid, owned) in grid_query.iter() {
        for (unit, location) in unit_query.iter_many(owned.iter()) {
            unit_gizmo(&mut gizmos, unit, grid_location(transform, grid, location));
        }
    }
}

fn unit_gizmo(gizmos: &mut Gizmos, unit: &unit::Unit, isometry: Isometry2d) {
    match unit.unit_type {
        unit::UnitType::Offensive => {
            gizmos.rect_2d(
                isometry,
                Vec2::new(UNIT_SCALE, UNIT_SCALE),
                unit_color(unit),
            );
        }
        unit::UnitType::Defensive => {
            gizmos.circle_2d(isometry, UNIT_SCALE * 0.5, unit_color(unit));
        }
        unit::UnitType::Mixed => {
            gizmos
                .rounded_rect_2d(
                    isometry,
                    Vec2::new(UNIT_SCALE, UNIT_SCALE),
                    unit_color(unit),
                )
                .corner_radius(UNIT_SCALE * 0.2);
        }
    }
}

fn unit_color(u: &unit::Unit) -> Color {
    match u.unit_group {
        unit::UnitGroup::Player => Color::srgb(0.0, 1.0, 0.0),
        unit::UnitGroup::Enemy => Color::srgb(1.0, 0.0, 0.0),
        unit::UnitGroup::Neutral => Color::srgb(1.0, 1.0, 0.0),
    }
}

fn turn_order_gizmo(
    mut gizmos: Gizmos,
    grid_query: Query<(&Transform, &grid::Grid, &game::TurnOrder)>,
    unit_query: Query<&unit::Unit>,
) {
    for (transform, grid, turn_order) in grid_query.iter() {
        let mut offset = 1;
        for turn in turn_order.iter_turns() {
            if let Ok(unit) = unit_query.get(*turn) {
                unit_gizmo(
                    &mut gizmos,
                    unit,
                    grid_location(transform, grid, grid.get_size() + IVec2::new(0, -offset)),
                );
                offset += 1;
            }
        }
    }
}
