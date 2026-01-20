use bevy::prelude::*;

use crate::theme;
use crate::util::cords;

const EFFECT_Z_LAYER: f32 = 900.0;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_effect);
    app.add_systems(Update, (process_effects, update_curves).chain());
}

#[derive(Event, Clone, Debug, Reflect)]
pub enum Effect {
    Swing(Vec2, Vec2),
    Shoot(Vec2, Vec2),
}

#[derive(Component)]
struct EffectTimer {
    pub timer: Timer,
}

impl EffectTimer {
    fn new(duration: f32) -> Self {
        EffectTimer {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

fn spawn_effect(trigger: On<Effect>, mut commands: Commands, sprites: Res<theme::Textures>) {
    let effect = trigger.event();
    commands.spawn(match effect {
        Effect::Swing(from, to) => {
            let from = from.extend(EFFECT_Z_LAYER);
            let to = to.extend(EFFECT_Z_LAYER);
            (
                EffectTimer::new(0.2),
                Transform::from_translation(from).with_rotation(cords::quad_to(from, to)),
                EffectTranslationCurves {
                    curves: vec![
                        EasingCurve::new(from, to, EaseFunction::CubicOut),
                        EasingCurve::new(to, from, EaseFunction::CubicIn),
                    ],
                },
                Sprite {
                    color: Color::linear_rgb(1.0, 0.0, 0.0),
                    ..sprites.swing.sprite()
                },
                Name::new("DamageEffect"),
            )
        }
        Effect::Shoot(from, to) => {
            let from = from.extend(EFFECT_Z_LAYER);
            let to = to.extend(EFFECT_Z_LAYER);
            (
                EffectTimer::new(0.2),
                Transform::from_translation(from).with_rotation(cords::quad_to(from, to)),
                EffectTranslationCurves {
                    curves: vec![EasingCurve::new(from, to, EaseFunction::Linear)],
                },
                Sprite {
                    color: Color::linear_rgb(0.0, 1.0, 0.0),
                    ..sprites.attack.sprite()
                },
                Name::new("DamageEffect"),
            )
        }
    });
}

fn process_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut EffectTimer)>,
) {
    for (entity, mut effect_instance) in query.iter_mut() {
        effect_instance.timer.tick(time.delta());
        if effect_instance.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct EffectTranslationCurves {
    curves: Vec<EasingCurve<Vec3>>,
}

fn update_curves(mut query: Query<(&mut Transform, &EffectTimer, &EffectTranslationCurves)>) {
    for (mut transform, timer, curves) in query.iter_mut() {
        let fraction = timer.timer.fraction();
        let index = (fraction * curves.curves.len() as f32).floor() as usize;
        if index < curves.curves.len() {
            transform.translation = curves.curves[index]
                .sample(fraction * curves.curves.len() as f32 - index as f32)
                .unwrap_or(transform.translation);
        }
    }
}
