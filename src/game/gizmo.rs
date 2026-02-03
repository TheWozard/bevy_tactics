use bevy::math::VectorSpace;
use bevy::prelude::*;

use super::grid;
use super::unit;

const HEALTH_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Update, unit_health_gizmo);
}

fn unit_health_gizmo(
    mut gizmos: Gizmos,
    unit_query: Query<(&Transform, &Sprite, &unit::Health), With<unit::Unit>>,
) {
    for (transform, sprite, health) in unit_query.iter() {
        let size = sprite.custom_size.unwrap_or(Vec2::ZERO);
        let location = transform.translation.xy()
            - Vec2::new(size.x / 2.0, size.y / 2.0)
            - Vec2::new(0.0, 10.0);
        let width = size.x;
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
