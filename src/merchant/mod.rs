use std::marker::PhantomData;
use std::time::Duration;

use bevy::input::common_conditions::input_just_pressed;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;

use crate::theme;

mod gizmo;
mod item;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            AppSystems::TickTimers,
            AppSystems::Prepare,
            AppSystems::Actions,
            AppSystems::PostActions,
        )
            .chain(),
    );

    app.add_plugins(gizmo::plugin);
    app.add_plugins(item::plugin);

    app.init_resource::<ShopOpenTimer>();
    app.add_event::<ShopOpen>();
    app.add_systems(
        Update,
        (ShopOpenTimer::start, ShopOpenTimer::tick)
            .in_set(AppSystems::TickTimers)
            .chain(),
    );

    app.add_plugins(TrackedResource::<Money>::plugin);
    app.add_systems(Startup, Money::spawn_ui);
    app.add_systems(
        FixedUpdate,
        Money::increment::<10>.run_if(input_pressed(KeyCode::KeyM)),
    );
    app.add_systems(
        FixedUpdate,
        Money::increment::<-10>.run_if(input_pressed(KeyCode::KeyN)),
    );
    app.add_systems(FixedUpdate, sell.run_if(input_just_pressed(KeyCode::Space)));
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    TickTimers,
    Prepare,
    Actions,
    PostActions,
}

#[derive(Resource)]
pub struct ShopOpenTimer(Timer);

impl Default for ShopOpenTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(60.0, TimerMode::Once);
        timer.pause();
        Self(timer)
    }
}

impl ShopOpenTimer {
    pub fn is_open(&self) -> bool {
        !self.0.finished() && !self.0.paused() && self.0.elapsed_secs() > 0.0
    }

    pub fn fraction(&self) -> f32 {
        if self.0.duration() == Duration::ZERO {
            0.0
        } else {
            self.0.elapsed_secs() / self.0.duration().as_secs_f32()
        }
    }

    // -- Systems --

    pub fn tick(mut timer: ResMut<Self>, time: Res<Time>) {
        timer.0.tick(time.delta());
    }

    pub fn start(
        mut timer: ResMut<Self>,
        input: Res<ButtonInput<KeyCode>>,
        mut commands: Commands,
    ) {
        if !timer.is_open() && input.just_pressed(KeyCode::Space) {
            timer.0.reset();
            timer.0.unpause();
            commands.trigger(ShopOpen);
        }
    }
}

#[derive(Event)]
pub struct ShopOpen;

#[derive(Resource, Default)]
pub struct Money(i32);

impl TrackableResource for Money {
    fn value(&self) -> i32 {
        self.0
    }
}

impl Money {
    pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            theme::baseline(),
            children![(
                theme::styled_span(),
                children![(
                    theme::text_style(&asset_server),
                    TrackedResource::<Money> {
                        name: "Money".to_string(),
                        ..default()
                    }
                )],
            )],
            Name::new("Money UI"),
        ));
    }

    pub fn increment<const AMOUNT: i32>(mut resource: ResMut<Money>) {
        resource.0 += AMOUNT;
    }
}

pub trait TrackableResource: Resource + Send + Sync {
    fn value(&self) -> i32;
}

#[derive(Component)]
pub struct TrackedResource<T: Resource + TrackableResource + Default> {
    pub name: String,
    phantom: PhantomData<T>,
}

impl<T: Resource + TrackableResource + Default> Default for TrackedResource<T> {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            phantom: PhantomData,
        }
    }
}

impl<T: Resource + TrackableResource + Default> TrackedResource<T> {
    fn text(&self, res: &T) -> String {
        format!("{}: {}", self.name, res.value())
    }

    fn update(mut query: Query<(&mut Text, &TrackedResource<T>)>, resource: Res<T>) {
        for (mut text, tracked) in query.iter_mut() {
            text.0 = tracked.text(&resource);
        }
    }

    fn observer(
        trigger: Trigger<OnAdd, Self>,
        resource: Res<T>,
        query: Query<&TrackedResource<T>>,
        mut commands: Commands,
    ) {
        if let Ok(tracked) = query.get(trigger.target()) {
            commands
                .entity(trigger.target())
                .insert(Text::new(tracked.text(&resource)));
        }
    }

    pub fn plugin(app: &mut App) {
        app.init_resource::<T>();
        app.add_systems(
            Update,
            Self::update
                .run_if(resource_changed::<T>)
                .in_set(AppSystems::PostActions),
        );
        app.add_observer(Self::observer);
    }
}

pub fn sell(query: Query<&item::Holding>) {
    for holding in query {
        log::info!("Selling item: {:?}", holding);
    }
}
