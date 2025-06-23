use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use super::grid;
use super::unit;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(
        PreUpdate,
        (TurnOrder::update_turn_order, TurnOrder::next_turn, TurnOrder::move_entity).chain(),
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
        TurnOrder { order: Vec::new(), index: 0 }
    }
}

impl TurnOrder {
    pub fn update_turn_order(mut turn_order: Query<(&mut TurnOrder, &grid::GridOwned), Changed<grid::GridOwned>>) {
        for (mut turn_order, grid_owned) in turn_order.iter_mut() {
            turn_order.order = grid_owned.iter().collect();
            turn_order.index = turn_order.index % turn_order.order.len();
        }
    }

    pub fn next_turn(keyboard_input: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<Turn>, mut turn_order: Query<&mut TurnOrder>) {
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
        mut unit_query: Query<(&mut grid::GridLocation, &mut unit::Movement, &grid::GridOwner, &unit::Unit)>,
        mut target_query: Query<(&mut unit::Health, &unit::Unit)>,
        mut grid_query: Query<&mut grid::Grid>,
        search_query: Query<&unit::Unit>,
    ) {
        for event in events.read() {
            if let Ok((mut grid_location, mut movement, grid_owner, unit)) = unit_query.get_mut(event.entity) {
                if let Ok(mut grid) = grid_query.get_mut(grid_owner.get()) {
                    let location = grid_location.as_ivec2();
                    if let Some(nearest) = grid.find(location, &movement.direction.as_ivec2(), |entity| {
                        if let Ok(search) = search_query.get(entity) {
                            search.team != unit.team
                        } else {
                            false
                        }
                    }) {
                        if nearest.distance_squared(*location) <= 1 {
                            if let Some(target) = grid.get(&nearest) {
                                if let Ok((mut health, target)) = target_query.get_mut(target) {
                                    if target.team == unit.team {
                                        movement.direction = Vec2::ZERO; // Stop moving if we hit an enemy
                                    } else {
                                        health.damage(1);
                                    }
                                }
                            }
                        } else {
                            if let Some(new_location) = grid.a_star_move(location, &nearest, movement.spaces as usize) {
                                movement.direction = (nearest - new_location.as_ivec2()).as_vec2().normalize_or_zero();
                                *grid_location = new_location;
                                break;
                            } else {
                                movement.direction = Vec2::ZERO; // No valid path, stop moving
                            }
                        }
                    } else {
                        movement.direction = Vec2::ZERO; // No valid target, stop moving
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
