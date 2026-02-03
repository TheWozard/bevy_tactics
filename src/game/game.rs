use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use super::grid;
use super::unit;
use crate::game::animate::Lerp;
use crate::game::effect;
use crate::util::cords;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(PreUpdate, TurnOrder::next_turn);

    app.add_observer(do_turn);
    app.add_observer(do_move);
    app.add_observer(do_attack);
    app.register_type::<TurnOrder>();
}

#[derive(EntityEvent, Clone, Debug, Reflect)]
pub struct Turn {
    entity: Entity,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct TurnOrder {
    pub order: Vec<Vec<Entity>>,
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
    pub fn add_entity(&mut self, entity: Entity, index: usize) {
        while self.order.len() < index + 1 {
            self.order.push(Vec::new());
        }
        self.order[index].push(entity);
    }

    pub fn add_entity_optional(&mut self, entity: Option<Entity>, index: usize) {
        if let Some(e) = entity {
            self.add_entity(e, index);
        }
    }

    pub fn next_turn(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut commands: Commands,
        mut turn_order: Query<&mut TurnOrder>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for mut turn in turn_order.iter_mut() {
                if let Some(next) = turn.get_next_entity() {
                    for entity in next {
                        commands.trigger(Turn {
                            entity: entity.clone(),
                        });
                    }
                }
            }
        }
    }

    pub fn get_next_entity(&mut self) -> Option<&Vec<Entity>> {
        if self.order.is_empty() {
            return None;
        }
        let current = self.index;
        self.index = (self.index + 1) % self.order.len();
        Some(&self.order[current])
    }
}

fn do_turn(
    trigger: On<Turn>,
    mut commands: Commands,
    unit_query: Query<(&grid::GridLocation, &unit::Unit, &unit::Attacks)>,
    target_query: Query<&unit::Unit>,
    grid_query: Query<&mut grid::Grid>,
) {
    if let Ok((location, unit, attacks)) = unit_query.get(trigger.event_target()) {
        if let Ok(grid) = grid_query.single() {
            if let Some(target_location) = grid.nearest_entity(
                &super::grid::EntityKind::Unit,
                location.location(),
                &IVec2::new(1, 0),
                grid::selection::Shape::All,
                |entity| {
                    target_query
                        .get(entity.clone())
                        .map_or(false, |u| u.team != unit.team)
                },
            ) {
                if target_location
                    .as_vec2()
                    .distance_squared(location.location().as_vec2())
                    <= attacks.range * attacks.range
                {
                    commands.trigger(Attack {
                        entity: trigger.event_target(),
                        target: grid
                            .get_entity(&super::grid::EntityKind::Unit, &target_location)
                            .unwrap(),
                    });
                } else {
                    commands.trigger(Move {
                        entity: trigger.event_target(),
                        towards: target_location,
                    });
                }
            }
        }
    }
}

#[derive(EntityEvent, Clone, Debug, Reflect)]
pub struct Move {
    entity: Entity,
    towards: IVec2,
}

fn do_move(
    trigger: On<Move>,
    mut commands: Commands,
    mut unit_query: Query<(
        &mut grid::GridLocation,
        &Transform,
        &unit::Movement,
        &grid::GridOwner,
    )>,
    mut grid_query: Query<(&mut grid::Grid, &grid::GridScale)>,
) {
    if let Ok((mut location, transform, movement, grid_owner)) =
        unit_query.get_mut(trigger.event_target())
    {
        if let Ok((mut grid, grid_scale)) = grid_query.get_mut(grid_owner.get()) {
            let steps = grid.a_star_next_to(
                &super::grid::EntityKind::Unit,
                location.location(),
                &trigger.event().towards,
                movement.spaces as usize,
            );
            if steps.len() > 1 {
                grid.move_to(&mut location, steps.last().unwrap());
                commands.entity(trigger.event_target()).insert((Lerp::new(
                    steps
                        .iter()
                        .map(|loc| {
                            cords::location_to_translation(
                                loc,
                                grid_scale.scale(),
                                transform.translation.z as i32,
                            )
                        })
                        .collect(),
                    0.2,
                ),));
            }
        }
    }
}

#[derive(EntityEvent, Clone, Debug, Reflect)]
pub struct Attack {
    entity: Entity,
    target: Entity,
}

fn do_attack(
    trigger: On<Attack>,
    unit_query: Query<(&Transform, &unit::Attacks)>,
    mut target_query: Query<(&Transform, &mut unit::Health)>,
    mut commands: Commands,
) {
    if let Ok((target_transform, mut health)) = target_query.get_mut(trigger.event().target) {
        if let Ok((source_transform, attacks)) = unit_query.get(trigger.event_target()) {
            health.damage(attacks.damage);
            let source = source_transform.translation.truncate();
            let target = target_transform.translation.truncate();
            if attacks.range <= 1.5 {
                commands.trigger(effect::Effect::Swing(
                    cords::percent_between(source, target, 0.25),
                    cords::percent_between(source, target, 0.75),
                ));
                return;
            } else {
                commands.trigger(effect::Effect::Shoot(
                    cords::percent_between(source, target, 0.25),
                    cords::percent_between(source, target, 0.75),
                ));
            }
        }
    }
}
