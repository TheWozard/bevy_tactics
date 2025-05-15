use crate::ui::material;
use bevy::prelude::*;
use rand::Rng;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, PedestalAnimation::tick_and_update);
}

pub struct Item {
    pub material: material::Material,
}

const COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
const PADDING: UiRect = UiRect::all(Val::Px(10.0));
const RADIUS: Val = Val::Px(5.0);
const SPACING: Val = Val::Px(20.0);

impl Item {
    // pub fn spawn(
    //     &self,
    //     commands: &mut Commands,
    //     assets: &AssetServer,
    //     hover: &popover::Details,
    // ) -> Entity {
    //     commands.spawn(self.bundle()).id()
    // }

    pub fn bundle(&self) -> impl Bundle {
        (
            Node {
                column_gap: SPACING,
                ..default()
            },
            children![(
                Node {
                    padding: PADDING,
                    ..default()
                },
                BorderRadius::all(RADIUS),
                BackgroundColor(COLOR),
                children![
                    PedestalAnimation::default().bundle(Val::Px(128.0), self.material.clone())
                ],
            ),],
        )
    }
}

#[derive(Component)]
pub struct PedestalAnimation {
    timer: Timer,
    curve: Box<dyn Curve<Vec3> + Send + Sync>,
}

impl Default for PedestalAnimation {
    fn default() -> Self {
        let mut rng = rand::rng();
        let mut animation = PedestalAnimation::new(
            Vec2::new(rng.random_range(5.0..9.0), rng.random_range(2.0..4.0)),
            rng.random_range(1.0..4.0),
            rng.random_range(2.0..4.0),
        );
        animation.timer.set_elapsed(
            animation
                .timer
                .duration()
                .mul_f32(rng.random_range(0.0..1.0)),
        );
        animation
    }
}

impl PedestalAnimation {
    pub fn new(scale: Vec2, angle: f32, seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
            curve: Box::new(FunctionCurve::new(
                Interval::new(0., 2. * std::f32::consts::PI).unwrap(),
                move |t| {
                    let (s, c) = t.sin_cos();
                    Vec3::new(s * scale.x, c * scale.y, s * angle)
                },
            )),
        }
    }

    pub fn tick_and_update(
        mut query: Query<(&mut PedestalAnimation, &mut Node, &mut Transform)>,
        time: Res<Time>,
    ) {
        for (mut animation, mut node, mut transform) in query.iter_mut() {
            animation.timer.tick(time.delta());
            if let Some(position) = animation
                .curve
                .sample(animation.timer.fraction() * animation.curve.domain().end())
            {
                node.left = Val::Px(position.x);
                node.bottom = Val::Px(position.y);
                transform.rotation = Quat::from_rotation_z(position.z.to_radians());
            }
        }
    }

    pub fn bundle(self, size: Val, item: material::Material) -> impl Bundle {
        (
            Node {
                width: size,
                height: size,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            children![(
                Node {
                    position_type: PositionType::Absolute,
                    height: size,
                    width: size,
                    ..default()
                },
                self,
                item,
            ),],
        )
    }
}
