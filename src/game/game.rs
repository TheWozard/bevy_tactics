use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use rand;

use super::grid;
use super::unit;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(
        PreUpdate,
        (
            TurnOrder::update_turn_order,
            TurnOrder::next_turn,
            TurnOrder::move_entity,
        )
            .chain(),
    );

    app.add_event::<Turn>();
    app.register_type::<TurnOrder>();
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct Turn {
    pub entity: Entity,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct TurnOrder {
    pub order: Vec<Entity>,
    pub index: usize,
}

impl Default for TurnOrder {
    fn default() -> Self {
        TurnOrder {
            order: Vec::new(),
            index: 0,
        }
    }
}

impl TurnOrder {
    pub fn update_turn_order(
        mut turn_order: Query<(&mut TurnOrder, &grid::GridOwned), Changed<grid::GridOwned>>,
    ) {
        for (mut turn_order, grid_owned) in turn_order.iter_mut() {
            turn_order.order = grid_owned.iter().collect();
        }
    }

    pub fn next_turn(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut event_writer: EventWriter<Turn>,
        mut turn_order: Query<&mut TurnOrder>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for mut turn in turn_order.iter_mut() {
                if let Some(entity) = turn.get_next_entity() {
                    event_writer.write(Turn { entity });
                }
            }
        }
    }

    pub fn move_entity(
        mut events: EventReader<Turn>,
        mut unit_query: Query<(&mut grid::GridLocation, &unit::Stats, &grid::GridOwner)>,
        mut grid_query: Query<&mut grid::Grid>,
    ) {
        for event in events.read() {
            if let Ok((mut grid_location, _stats, grid_owner)) = unit_query.get_mut(event.entity) {
                if let Ok(grid) = grid_query.get_mut(grid_owner.get()) {
                    loop {
                        let mut location = grid_location.as_ivec2();
                        match rand::random_range(0..4) {
                            0 => location.x += 1,
                            1 => location.x -= 1,
                            2 => location.y += 1,
                            _ => location.y -= 1,
                        }
                        if let Some(new_location) = grid.get_location(location) {
                            *grid_location = new_location;
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn get_next_entity(&mut self) -> Option<Entity> {
        if self.order.is_empty() {
            return None;
        }
        let current = self.index;
        self.index = (self.index + 1) % self.order.len();
        Some(self.order[current])
    }

    pub fn iter_turns(&self) -> impl Iterator<Item = &Entity> {
        let append = self.order.iter().take(self.index);
        self.order.iter().skip(self.index).chain(append)
    }
}
