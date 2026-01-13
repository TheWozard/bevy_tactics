use bevy::prelude::*;

#[inline]
pub fn rotation_to(from: Vec3, to: Vec3) -> f32 {
    let direction = to - from;
    direction.y.atan2(direction.x)
}

#[inline]
pub fn quad_to(from: Vec3, to: Vec3) -> Quat {
    Quat::from_rotation_z(rotation_to(from, to))
}

#[inline]
pub fn percent_between(from: Vec2, to: Vec2, percent: f32) -> Vec2 {
    from + (to - from) * percent
}
