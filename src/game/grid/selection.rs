use bevy::prelude::*;

use crate::random::RandomSource;

#[derive(Clone, Debug, Reflect)]
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
                location.x >= start.x
                    && location.x < end.x
                    && location.y >= start.y
                    && location.y < end.y
            }
        }
    }

    pub fn random(&self, source: &mut RandomSource) -> IVec2 {
        match self {
            Shape::All => IVec2::ZERO,
            Shape::Circle(center, radius) => {
                let angle = source.random::<f32>() * std::f32::consts::TAU;
                let r = source.random::<f32>() * radius;
                IVec2::new(
                    (center.x + r * angle.cos()).round() as i32,
                    (center.y + r * angle.sin()).round() as i32,
                )
            }
            Shape::Square(start, end) => IVec2::new(
                source.random::<i32>().rem_euclid(end.x - start.x) + start.x,
                source.random::<i32>().rem_euclid(end.y - start.y) + start.y,
            ),
        }
    }
}
