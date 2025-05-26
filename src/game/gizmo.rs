use bevy::prelude::*;

use super::game;
use super::grid;
use super::unit;

const GRID_SCALE: Vec2 = Vec2::new(40., 40.);
const UNIT_SCALE: f32 = 30.;
const GRID_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const HEALTH_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(
        Update,
        (grid_gizmo, units_gizmo, turn_order_gizmo, unit_health_gizmo),
    );
}

fn grid_gizmo(mut gizmos: Gizmos, query: Query<(&Transform, &grid::Grid)>) {
    for (transform, grid) in query.iter() {
        gizmos.grid_2d(
            Isometry2d::from_translation(transform.translation.truncate()),
            grid.get_size().as_uvec2(),
            GRID_SCALE,
            GRID_COLOR.with_alpha(0.2),
        );
        gizmos.rect_2d(
            Isometry2d::from_translation(transform.translation.truncate()),
            grid.get_size().as_vec2() * GRID_SCALE,
            GRID_COLOR,
        );
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
    unit_query: Query<(&unit::Unit, &grid::GridLocation, Option<&unit::Movement>)>,
    grid_query: Query<(&Transform, &grid::Grid, &grid::GridOwned)>,
) {
    for (transform, grid, owned) in grid_query.iter() {
        for (unit, location, movement) in unit_query.iter_many(owned.iter()) {
            let loc = grid_location(transform, grid, location);
            if let Some(movement) = movement {
                unit_gizmo(&mut gizmos, unit, loc.clone());
                if movement.direction != Vec2::ZERO {
                    gizmos.arrow_2d(
                        loc.translation - (movement.direction * UNIT_SCALE * 0.2),
                        loc.translation + (movement.direction * UNIT_SCALE * 0.2),
                        unit.color,
                    );
                }
            }
        }
    }
}

fn unit_gizmo(gizmos: &mut Gizmos, unit: &unit::Unit, isometry: Isometry2d) {
    gizmos
        .circle_2d(isometry, UNIT_SCALE * 0.5, unit.color)
        .resolution(unit.sides);
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
                    grid_location(transform, grid, grid.get_size() + IVec2::new(1, -offset)),
                );
                offset += 1;
            }
        }
    }
}

fn unit_health_gizmo(
    mut gizmos: Gizmos,
    unit_query: Query<(&grid::GridLocation, &unit::Health), With<unit::Unit>>,
    grid_query: Query<(&Transform, &grid::Grid, &grid::GridOwned)>,
) {
    for (transform, grid, owned) in grid_query.iter() {
        for (location, health) in unit_query.iter_many(owned.iter()) {
            let width = UNIT_SCALE;
            let location = grid_location(transform, grid, location).translation
                + Vec2::new(-(width * 0.5), UNIT_SCALE * 0.6);
            let percent = health.percent();
            gizmos.line_2d(
                location,
                location + Vec2::new(width * percent, 0.0),
                HEALTH_COLOR,
            );
            if percent < 1.0 {
                gizmos.line_2d(
                    location + Vec2::new(width * percent, 0.0),
                    location + Vec2::new(width, 0.0),
                    HEALTH_COLOR.with_alpha(0.2),
                );
            }
        }
    }
}
