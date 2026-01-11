use bevy::prelude::*;

use super::grid;
use super::unit;
use crate::theme;

const HEALTH_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Update, unit_health_gizmo);
}

fn grid_location(
    transform: &Transform,
    grid: &grid::Grid,
    grid_location: impl Into<IVec2>,
    scale: f32,
) -> Isometry2d {
    let offset =
        (grid.grid.size().as_vec2() - 1.0) * 0.5 * scale + transform.translation.truncate();
    Isometry2d::from_translation(grid_location.into().as_vec2() * scale - offset)
}

fn unit_health_gizmo(
    mut gizmos: Gizmos,
    unit_query: Query<(&grid::GridLocation, &unit::Health), With<unit::Unit>>,
    grid_query: Query<(&Transform, &grid::Grid, &grid::GridOwned)>,
    sprites: Res<theme::Sprites>,
) {
    for (transform, grid, owned) in grid_query.iter() {
        for (location, health) in unit_query.iter_many(owned.iter()) {
            let width = sprites.scale;
            let location = grid_location(transform, grid, location, sprites.scale).translation
                + Vec2::new(-(width * 0.5), sprites.scale * 0.6);
            let percent = health.percent();
            if percent < 1.0 {
                gizmos.line_2d(
                    location,
                    location + Vec2::new(width * percent, 0.0),
                    HEALTH_COLOR,
                );
                gizmos.line_2d(
                    location + Vec2::new(width * percent, 0.0),
                    location + Vec2::new(width, 0.0),
                    HEALTH_COLOR.with_alpha(0.2),
                );
            }
        }
    }
}
