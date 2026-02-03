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

// Index

#[inline]
// Converts a linear index to a 2D grid location.
pub fn index_to_location(size: &IVec2, index: usize) -> IVec2 {
    IVec2::new(
        (index % size.x as usize) as i32,
        (index / size.x as usize) as i32,
    )
}

// Location

#[inline]
// Converts a 2D grid location to a linear index.
pub fn location_to_index(size: &IVec2, location: &IVec2) -> usize {
    (location.y * size.x + location.x) as usize
}

#[inline]
// Checks if a 2D grid location is within the bounds of the grid.
pub fn location_within(start: &IVec2, end: &IVec2, location: &IVec2) -> bool {
    location.x >= start.x && location.x < end.x && location.y >= start.y && location.y < end.y
}

// Translation

#[inline]
// Converts a 2D grid location to a 3D world position.
pub fn location_to_translation(location: &IVec2, scale: &IVec2, z: i32) -> Vec3 {
    (location * scale).extend(z).as_vec3()
}
