use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::camera::camera_system;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;
use bevy::sprite_render::Material2dPlugin;

use crate::random::RandomSource;

pub fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, scale_background.after(camera_system));

    app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
    app.add_observer(spawn_background);
}

const SHADER_ASSET_PATH: &str = "shaders/background.wgsl";

fn spawn_background(
    event: On<Add, Camera2d>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    mut rand: ResMut<RandomSource>,
) {
    let entity = event.event_target();
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(BackgroundMaterial::new(
            rand.as_mut(),
            Color::srgb(0.0, 1.0, 0.0),
            Color::srgb(0.0, 0.0, 0.0),
        ))),
        Transform::from_translation(Vec2::ZERO.extend(-100.)),
        Name::new("Background"),
        ChildOf(entity),
    ));
}

fn scale_background(
    mut query: Query<&Projection, Changed<Projection>>,
    mut background: Query<&mut Transform, With<MeshMaterial2d<BackgroundMaterial>>>,
) {
    for proj in query.iter_mut() {
        let mut transform = background.single_mut().unwrap();
        if let Projection::Orthographic(ortho) = proj {
            let area = ortho.area.size();
            let size = area.x.max(area.y);
            *transform = transform.with_scale(Vec2::splat(size).extend(1.0));
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct BackgroundMaterial {
    #[uniform(0)]
    seed: f32,
    #[uniform(1)]
    highlight: LinearRgba,
    #[uniform(2)]
    base: LinearRgba,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

impl BackgroundMaterial {
    fn new(rand: &mut RandomSource, highlight: Color, base: Color) -> Self {
        BackgroundMaterial {
            seed: rand.random(),
            highlight: highlight.to_linear(),
            base: base.to_linear(),
        }
    }
}
