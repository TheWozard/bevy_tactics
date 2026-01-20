use bevy::prelude::*;

// Defines a subsection of a grid to iterate over. Used to optimize search areas.
pub enum Shape {
    All,
    Circle(Vec2, f32),
    Square(IVec2, IVec2),
}

impl Shape {
    pub fn contains(&self, location: &IVec2) -> bool {
        match self {
            Shape::All => true,
            Shape::Circle(center, radius) => {
                center.distance_squared(location.as_vec2()) <= radius * radius
            }
            Shape::Square(start, end) => {
                location.x >= start.x && location.x < end.x &&
                location.y >= start.y && location.y < end.y
            }
        }
    }
}