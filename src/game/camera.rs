use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(CameraControls::default());
    app.add_observer(move_camera_event);

    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, control_camera);
    app.add_systems(Update, camera_zoom);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(ControlledCamera::new());
}

#[derive(Resource, Clone, Debug, Reflect)]
pub struct CameraControls {
    pub drag: f32,
    pub acceleration: f32,
    pub max: Vec3,
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

impl CameraControls {
    pub fn default() -> Self {
        CameraControls {
            drag: 0.005,
            acceleration: 2000.0,
            max: Vec2::splat(500.0).extend(0.0),
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Camera2d)]
pub struct ControlledCamera {
    velocity: Vec3,
}

impl ControlledCamera {
    pub fn new() -> Self {
        ControlledCamera {
            velocity: Vec3::ZERO,
        }
    }

    fn reset(&mut self) {
        self.velocity = Vec3::ZERO;
    }
}

fn control_camera(
    mut query: Query<(&mut Transform, &mut ControlledCamera)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    controls: Res<CameraControls>,
) {
    for (mut transform, mut controller) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(controls.up) {
            direction.y += 1.0;
        }
        if input.pressed(controls.down) {
            direction.y -= 1.0;
        }
        if input.pressed(controls.left) {
            direction.x -= 1.0;
        }
        if input.pressed(controls.right) {
            direction.x += 1.0;
        }
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            controller.velocity += direction * controls.acceleration * time.delta_secs();
            controller.velocity = controller.velocity.clamp(-controls.max, controls.max);
            transform.translation += controller.velocity * time.delta_secs();
        } else {
            if controller.velocity.length_squared() < 0.01 {
                controller.velocity = Vec3::ZERO;
            } else {
                transform.translation += controller.velocity * time.delta_secs();
                controller.velocity *= controls.drag.powf(time.delta_secs());
            }
        }
    }
}

fn camera_zoom(
    mut input: MessageReader<MouseWheel>,
    mut query: Query<&mut Projection, With<ControlledCamera>>,
) {
    for mut camera in query.iter_mut() {
        match &mut *camera {
            Projection::Orthographic(ortho) => {
                for event in input.read() {
                    let scroll_amount = match event.unit {
                        MouseScrollUnit::Line => event.y * 0.1,
                        MouseScrollUnit::Pixel => event.y * 0.001,
                    };
                    ortho.scale -= scroll_amount;
                    ortho.scale = ortho.scale.clamp(1.0, 2.0);
                }
            }
            _ => {}
        }
    }
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct MoveCameraEvent {
    target: Vec2,
}

impl MoveCameraEvent {
    pub fn new(target: Vec2) -> Self {
        MoveCameraEvent { target }
    }
}

fn move_camera_event(
    event: On<MoveCameraEvent>,
    mut query: Query<(&mut Transform, &mut ControlledCamera)>,
) {
    for (mut transform, mut controller) in query.iter_mut() {
        transform.translation = event.target.extend(transform.translation.z);
        controller.reset();
    }
}
