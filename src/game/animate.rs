use bevy::prelude::*;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(PreUpdate, Lerp::update);
    app.register_type::<Lerp>();
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform)]
pub struct Lerp {
    steps: Vec<EasingCurve<Vec3>>,
    index: usize,
    end: Vec3,
    timer: Timer,
}

impl Lerp {
    pub fn new(locations: Vec<Vec3>, duration: f32) -> Self {
        let mut steps = Vec::with_capacity(locations.len() - 1);
        for i in 0..(locations.len() - 1) {
            steps.push(EasingCurve::new(
                locations[i],
                locations[i + 1],
                EaseFunction::CubicOut,
            ));
        }
        Lerp {
            steps,
            index: 0,
            end: *locations.last().unwrap(),
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }

    pub fn update(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Lerp, &mut Transform)>,
        delta: Res<Time>,
    ) {
        for (entity, mut lerp, mut transform) in query.iter_mut() {
            lerp.timer.tick(delta.delta());
            if lerp.timer.just_finished() {
                lerp.index += 1;
            }
            if lerp.index >= lerp.steps.len() {
                transform.translation = lerp.end;
                commands.entity(entity).remove::<Lerp>();
            } else {
                transform.translation = lerp.steps[lerp.index]
                    .sample(lerp.timer.fraction())
                    .unwrap_or(lerp.end);
            }
        }
    }
}
