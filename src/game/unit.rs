use bevy::prelude::*;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(PostUpdate, despawn_on_zero_health);

    app.register_type::<Unit>();
    app.register_type::<Movement>();
    app.register_type::<Health>();
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform, Name::new("Unit"))]
pub struct Unit {
    pub team: u32,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Movement {
    pub spaces: u32,
}

impl Movement {
    pub fn new(spaces: u32) -> Self {
        Movement { spaces }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Health { current: max, max }
    }

    pub fn damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    pub fn percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

fn despawn_on_zero_health(
    mut commands: Commands,
    mut query: Query<(Entity, &Health), Changed<Health>>,
) {
    for (entity, health) in query.iter_mut() {
        if health.current == 0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Attacks {
    pub damage: u32,
    pub range: f32,
}

impl Attacks {
    pub fn new(damage: u32, range: u32) -> Self {
        Attacks {
            damage,
            range: range as f32,
        }
    }
}
