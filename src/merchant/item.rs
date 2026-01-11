use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::random::RandomSource;
use crate::theme::Sprites;
use crate::theme::{self};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, initial_items);
    app.add_observer(recolor_on::<Pointer<Over>, Item>(Color::srgb(
        0.0, 1.0, 1.0,
    )));
    app.add_observer(recolor_on::<Pointer<Out>, Item>(Color::srgb(1.0, 1.0, 1.0)));

    app.add_plugins(Holding::plugin);
    app.add_plugins(Drag::<Item, DropLocation, Holding>::plugin);
}

fn initial_items(mut commands: Commands, sprites: Res<Sprites>, mut rand: ResMut<RandomSource>) {
    for l in theme::grid(2, 5, Vec2::new(-200.0, 0.0), 300) {
        commands.spawn((
            Sprite::from_color(Color::hsv(0.0, 0.0, 1.0), Vec2::splat(64.0)),
            Transform::from_translation(l.extend(0.0)),
            Name::new("Location"),
            Pickable::default(),
            DropLocation,
        ));
    }

    for l in theme::grid(5, 5, Vec2::new(200.0, 0.0), 300) {
        if rand.ratio(2, 3) {
            commands.spawn((
                sprites.bundle(rand.range(16 * 3..16 * 40)),
                Transform::from_translation(l.extend(2.0)),
                Name::new("Item"),
                Pickable::default(),
                Item,
            ));
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
struct Item;

#[derive(Component, Debug, Clone, Reflect)]
struct DropLocation;

// An observer listener that changes the target entity's color.
fn recolor_on<E: Debug + Clone + Reflect, T: Component>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&mut Sprite, With<T>>) {
    move |ev, mut sprites| {
        if let Ok(mut sprite) = sprites.get_mut(ev.target()) {
            sprite.color = color;
        }
    }
}

#[derive(Resource, Debug, Clone, Reflect)]
struct Drag<U: Component, D: Component, R: Relationship> {
    target: Option<Entity>,
    _marker: PhantomData<(U, D, R)>,
}

impl<U: Component, D: Component, R: Relationship> Drag<U, D, R> {
    fn plugin(app: &mut App) {
        app.add_observer(Self::start_drag);
        app.add_observer(Self::end_drag);
        app.insert_resource(Self {
            target: None,
            _marker: PhantomData,
        });
    }

    fn start_drag(
        event: Trigger<Pointer<Pressed>>,
        mut drag: ResMut<Self>,
        query: Query<Entity, With<U>>,
    ) {
        if let Ok(_) = query.get(event.target()) {
            drag.target = Some(event.target());
        }
    }

    fn end_drag(
        event: Trigger<Pointer<Released>>,
        mut commands: Commands,
        drag: Res<Self>,
        query: Query<Entity, With<D>>,
    ) {
        if let Some(dragging) = drag.target {
            if let Ok(_) = query.get(event.target()) {
                commands
                    .entity(dragging)
                    .replace_related::<R>(&[event.target()]);
            }
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = HeldBy)]
pub struct Holding(Entity);

#[derive(Component, Debug, Clone, Reflect)]
#[relationship_target(relationship = Holding)]
pub struct HeldBy(Entity);

impl Holding {
    fn plugin(app: &mut App) {
        app.add_systems(Update, Self::update_location);
    }

    fn update_location(
        query: Query<(&Holding, &GlobalTransform), Changed<Holding>>,
        mut target: Query<&mut Transform>,
    ) {
        for (holding, holding_transform) in query {
            log::info! {
                "Updating location of holding: {:?} at position: {:?}",
                holding.0, holding_transform.translation()
            }
            if let Ok(mut target_transform) = target.get_mut(holding.0) {
                log::info!("Updating target transform: {:?}", target_transform);
                target_transform.translation.x = holding_transform.translation().x;
                target_transform.translation.y = holding_transform.translation().y;
                // target_transform.translation.z = target_transform.translation.z;
            }
        }
    }
}
