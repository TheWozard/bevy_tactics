use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;
use bevy::sprite_render::Material2dPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
    app.add_observer(spawn_background);
}

const SHADER_ASSET_PATH: &str = "shaders/background.wgsl";

fn spawn_background(
    event: On<Add, Camera2d>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let entity = event.event_target();
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(BackgroundMaterial::new(
            Color::srgb(0.0, 1.0, 0.0),
            Color::srgb(0.0, 0.0, 0.0),
        ))),
        Transform::default().with_scale(Vec3::splat(10000.)),
        Name::new("Background"),
        ChildOf(entity),
    ));
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct BackgroundMaterial {
    #[uniform(0)]
    highlight: LinearRgba,
    #[uniform(1)]
    base: LinearRgba,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

impl BackgroundMaterial {
    fn new(highlight: Color, base: Color) -> Self {
        BackgroundMaterial {
            highlight: highlight.to_linear(),
            base: base.to_linear(),
        }
    }
}
