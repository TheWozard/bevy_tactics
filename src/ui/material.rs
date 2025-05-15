use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(UiMaterialPlugin::<Glint>::default());
    app.add_systems(Update, animate::<Glint>);
    app.add_plugins(UiMaterialPlugin::<Distortion>::default());
    app.add_systems(Update, animate::<Distortion>);

    app.add_systems(PostUpdate, Material::spawn_material);
}

#[derive(Clone, Debug, Default)]
pub enum Type {
    #[default]
    None,
    Glint,
    Distortion,
}

#[derive(Component, Clone, Debug, Default)]
pub struct Material {
    pub material: Type,
    pub tint: Color,
    pub direction: Vec2,
    pub texture: Handle<Image>,
    pub specular_map: Option<Handle<Image>>,
}

impl Material {
    // spawn_material adds the MaterialNode to the entity with the ItemMaterial component.
    fn spawn_material(
        mut commands: Commands,
        assets: Res<AssetServer>,
        query: Query<(Entity, &Material), Added<Material>>,
        mut glint_assets: ResMut<Assets<Glint>>,
        mut distortion_assets: ResMut<Assets<Distortion>>,
    ) {
        for (entity, item_material) in query.iter() {
            let mut ec = commands.entity(entity);
            match item_material.material {
                Type::None => {
                    ec.insert(
                        ImageNode::new(item_material.texture.clone())
                            .with_color(item_material.tint),
                    );
                }
                Type::Glint => {
                    ec.insert(MaterialNode(glint_assets.add(Glint {
                        tint: item_material.tint.to_linear().to_vec4(),
                        time: 0.0,
                        texture: item_material.texture.clone(),
                        specular_map: item_material.get_specular_map(),
                    })));
                }
                Type::Distortion => {
                    ec.insert(MaterialNode(distortion_assets.add(Distortion {
                        tint: item_material.tint.to_linear().to_vec4(),
                        time: 0.0,
                        direction: item_material.direction,
                        texture: item_material.texture.clone(),
                        specular_map: item_material.get_specular_map(),
                        noise_texture: assets.load("images/noise_flame.png"),
                    })));
                }
            }
        }
    }

    // get_specular_map returns the specular map if it exists, otherwise it returns the texture.
    fn get_specular_map(&self) -> Handle<Image> {
        if self.specular_map.is_some() {
            self.specular_map.clone().unwrap()
        } else {
            self.texture.clone()
        }
    }
}

trait Animated {
    fn tick(&mut self, time: f32);
}

fn animate<T: Animated + UiMaterial>(
    mut materials: ResMut<Assets<T>>,
    query: Query<&MaterialNode<T>>,
    time: Res<Time>,
) {
    for node in query.iter() {
        if let Some(material) = materials.get_mut(node) {
            material.tick(time.delta_secs());
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct Glint {
    #[uniform(0)]
    tint: Vec4,

    #[uniform(1)]
    time: f32,

    #[texture(2)]
    #[sampler(3)]
    texture: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    specular_map: Handle<Image>,
}

impl UiMaterial for Glint {
    fn fragment_shader() -> ShaderRef {
        "shaders/glint_material.wgsl".into()
    }
}

impl Animated for Glint {
    fn tick(&mut self, time: f32) {
        self.time += time;
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct Distortion {
    #[uniform(0)]
    tint: Vec4,

    #[uniform(1)]
    time: f32,

    #[uniform(2)]
    direction: Vec2,

    #[texture(3)]
    #[sampler(4)]
    texture: Handle<Image>,

    #[texture(5)]
    #[sampler(6)]
    specular_map: Handle<Image>,

    #[texture(7)]
    #[sampler(8)]
    noise_texture: Handle<Image>,
}

impl UiMaterial for Distortion {
    fn fragment_shader() -> ShaderRef {
        "shaders/distortion_material.wgsl".into()
    }
}

impl Animated for Distortion {
    fn tick(&mut self, time: f32) {
        self.time += time;
    }
}
