use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use super::grid;
use super::unit;
use crate::game::animate::Lerp;
use crate::game::effect;
use crate::game::utils;
use crate::theme;
use crate::util::cords;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(
        PreUpdate,
        (TurnOrder::update_turn_order, TurnOrder::next_turn),
    );

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
    pub fn update_turn_order(
        mut turn_order: Query<(&mut TurnOrder, &grid::GridOwned), Changed<grid::GridOwned>>,
        speed: Query<&unit::Speed>,
    ) {
        for (mut turn_order, grid_owned) in turn_order.iter_mut() {
            turn_order.order.clear();
            let mut ordered = Vec::new();
            for entity in grid_owned.iter() {
                if let Ok(speed) = speed.get(entity) {
                    while ordered.len() < speed.value as usize {
                        ordered.push(Vec::new());
                    }
                    ordered[(speed.value - 1) as usize].push(entity);
                }
            }
            turn_order.order = vec![Vec::<Entity>::new(); ordered.len()];
            for (i, entities) in ordered.iter().enumerate() {
                if entities.len() > 0 {
                    turn_order.order[i].extend(entities.iter());
                    let mut multiple = 2;
                    while multiple * (i + 1) < turn_order.order.len() {
                        turn_order.order[multiple * (i + 1)].extend(entities.iter());
                        multiple += 1;
                    }
                }
            }
            turn_order.order.retain(|v| !v.is_empty());
            turn_order.index = turn_order.index % turn_order.order.len();
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

    // pub fn do_turn(
    //     trigger: On<Turn>,
    //     mut commands: Commands,
    //     mut unit_query: Query<(
    //         Entity,
    //         &Transform,
    //         &mut grid::GridLocation,
    //         &unit::Movement,
    //         &grid::GridOwner,
    //         &unit::Unit,
    //         &unit::Attacks,
    //     )>,
    //     mut target_query: Query<(&mut unit::Health, &unit::Unit)>,
    //     mut grid_query: Query<&mut grid::Grid>,
    //     search_query: Query<&unit::Unit>,
    //     sprites: Res<theme::Sprites>,
    // ) {
    //     let event = trigger.event();
    //     if let Ok((entity, transform, mut grid_location, movement, grid_owner, unit, attacks)) =
    //         unit_query.get_mut(event.entity)
    //     {
    //         if let Ok(mut grid) = grid_query.get_mut(grid_owner.get()) {
    //             let location = grid_location.as_ivec2();
    //             if let Some(nearest) = grid.find(location, &IVec2::ONE, |entity| {
    //                 if let Ok(search) = search_query.get(entity) {
    //                     search.team != unit.team
    //                 } else {
    //                     false
    //                 }
    //             }) {
    //                 if nearest.distance_squared(*location) <= 1 {
    //                     if let Some(target) = grid.get(&nearest) {
    //                         if let Ok((mut health, target)) = target_query.get_mut(target) {
    //                             if target.team != unit.team {
    //                                 health.damage(1 * attacks.damage);
    //                                 let source = transform.translation.truncate();
    //                                 let target =
    //                                     grid.grid.location_to_vec2(&nearest, sprites.scale);
    //                                 commands.trigger(effect::Effect::Damage(
    //                                     cords::percent_between(source, target, 0.25),
    //                                     cords::percent_between(source, target, 0.75),
    //                                 ));
    //                             }
    //                         }
    //                     }
    //                 } else {
    //                     if let Some((location, steps)) =
    //                         grid.a_star_move(location, &nearest, movement.spaces as usize)
    //                     {
    //                         commands.entity(entity).insert(Lerp::new(
    //                             steps
    //                                 .iter()
    //                                 .map(|loc| {
    //                                     grid.grid
    //                                         .location_to_vec2(loc, sprites.scale)
    //                                         .extend(transform.translation.z)
    //                                 })
    //                                 .collect(),
    //                             0.2,
    //                         ));
    //                         *grid_location = location;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

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
            if let Some(target_location) = grid.find(
                location.as_ivec2(),
                &IVec2::ONE,
                utils::SelectionShape::All,
                |entity| {
                    if let Ok(target_unit) = target_query.get(entity) {
                        target_unit.team != unit.team
                    } else {
                        false
                    }
                },
            ) {
                if target_location
                    .as_vec2()
                    .distance_squared(location.as_vec2())
                    <= attacks.range * attacks.range
                {
                    commands.trigger(Attack {
                        entity: trigger.event_target(),
                        target: grid.get(&target_location).unwrap(),
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
    unit_query: Query<(
        &Transform,
        &grid::GridLocation,
        &unit::Movement,
        &grid::GridOwner,
    )>,
    mut grid_query: Query<&mut grid::Grid>,
    sprites: Res<theme::Sprites>,
) {
    if let Ok((transform, location, movement, grid_owner)) = unit_query.get(trigger.event_target())
    {
        if let Ok(mut grid) = grid_query.get_mut(grid_owner.get()) {
            if let Some((new_location, steps)) = grid.a_star_move(
                location.as_ivec2(),
                &trigger.event().towards,
                movement.spaces as usize,
            ) {
                commands.entity(trigger.event_target()).insert((
                    new_location,
                    Lerp::new(
                        steps
                            .iter()
                            .map(|loc| {
                                grid.grid
                                    .location_to_vec2(loc, sprites.scale)
                                    .extend(transform.translation.z)
                            })
                            .collect(),
                        0.2,
                    ),
                ));
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
