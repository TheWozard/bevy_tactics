use bevy::{
    platform::collections::HashMap,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

pub fn plugin(app: &mut bevy::prelude::App) {
    app.init_resource::<DynamicImages>();
    app.add_event::<SkillTreeSpawnEvent>();
    app.add_systems(Startup, spawn_initial_tree);
    app.add_systems(Update, spawn_skill_tree);
}

pub fn spawn_initial_tree(mut events: EventWriter<SkillTreeSpawnEvent>) {
    events.write(SkillTreeSpawnEvent { tree: example_skill_tree() });
}

const UI_NODE_SCALE: f32 = 32.0;
const UI_EDGE_SCALE: f32 = 8.0;
const UI_NODE_SPACING: f32 = 100.0;
const UI_TEXTURE_MULTIPLIER: f32 = 2.0;
const UI_TEXTURE_SCALE: f32 = 1.0 / UI_TEXTURE_MULTIPLIER;

pub fn spawn_skill_tree(
    mut events: EventReader<SkillTreeSpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<DynamicImages>,
    mut assets: ResMut<Assets<Image>>,
) {
    let node_mesh = meshes.add(Rectangle::from_size(Vec2::new(1.0, 1.0)));
    let edge_mesh = meshes.add(Rectangle::from_size(Vec2::new(1.0, 1.0)));
    for event in events.read() {
        for edge in &event.tree.edges {
            let start_node = event.tree.nodes.get(&edge.0).unwrap();
            let start_node_position = start_node.position * UI_NODE_SPACING;
            let end_node = event.tree.nodes.get(&edge.1).unwrap();
            let end_node_position = end_node.position * UI_NODE_SPACING;
            let direction = end_node_position - start_node_position;
            let color = start_node.color.mix(&end_node.color, 0.5);

            match edge.2 {
                EdgeStyle::Straight => {
                    commands.spawn((
                        Transform::from_translation(((start_node_position + end_node_position) / 2.0).extend(1.0))
                            .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x)))
                            .with_scale(Vec3::new(direction.length(), UI_EDGE_SCALE, 1.0)),
                        Mesh2d(edge_mesh.clone()),
                        MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
                        Name::new("SkillTreeStraightEdge"),
                    ));
                }
                EdgeStyle::Curved(center) => {
                    let radius = Vec2::new(start_node_position.distance(center), end_node_position.distance(center));
                    commands.spawn((
                        Transform::from_translation(center.extend(1.0)).with_scale(Vec3::new(UI_TEXTURE_SCALE, UI_TEXTURE_SCALE, 1.0)),
                        Sprite {
                            image: images.get_circle(
                                (radius * UI_TEXTURE_MULTIPLIER).as_uvec2(),
                                UI_EDGE_SCALE * UI_TEXTURE_MULTIPLIER,
                                &mut assets,
                            ),
                            color: color,
                            ..default()
                        },
                        Name::new("SkillTreeCurvedEdge"),
                    ));
                }
            }
        }
        for node in &event.tree.nodes {
            commands.spawn((
                Transform::from_translation((node.1.position * UI_NODE_SPACING).extend(2.0)).with_scale(Vec3::new(UI_NODE_SCALE, UI_NODE_SCALE, 1.0)),
                Mesh2d(node_mesh.clone()),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(node.1.color))),
                Name::new("SkillTreeNode"),
            ));
        }
    }
}

#[derive(Event)]
pub struct SkillTreeSpawnEvent {
    pub tree: Tree<SkillTreeNode>,
}

pub enum EdgeStyle {
    Straight,
    Curved(Vec2),
}

pub struct Tree<T> {
    pub nodes: HashMap<SkillTreeNodeName, T>,
    pub edges: Vec<(SkillTreeNodeName, SkillTreeNodeName, EdgeStyle)>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum SkillTreeNodeName {
    InvalidNode,

    WhiteNode,
    RedNode,
    OrangeNode,
    YellowNode,
    GreenNode,
}

pub struct SkillTreeNode {
    pub position: Vec2,
    pub color: Color,
}

pub fn example_skill_tree() -> Tree<SkillTreeNode> {
    Tree {
        nodes: HashMap::from([
            (
                SkillTreeNodeName::WhiteNode,
                SkillTreeNode {
                    position: Vec2::new(0.0, 0.0),
                    color: Color::hsv(0.0, 0.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::RedNode,
                SkillTreeNode {
                    position: Vec2::new(1.0, 0.0),
                    color: Color::hsv(0.0, 1.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::OrangeNode,
                SkillTreeNode {
                    position: Vec2::new(0.0, 2.0),
                    color: Color::hsv(30.0, 1.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::YellowNode,
                SkillTreeNode {
                    position: Vec2::new(-1.0, 0.0),
                    color: Color::hsv(60.0, 1.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::GreenNode,
                SkillTreeNode {
                    position: Vec2::new(0.0, -2.0),
                    color: Color::hsv(120.0, 1.0, 1.0),
                },
            ),
        ]),
        edges: vec![
            (SkillTreeNodeName::WhiteNode, SkillTreeNodeName::RedNode, EdgeStyle::Straight),
            (SkillTreeNodeName::WhiteNode, SkillTreeNodeName::OrangeNode, EdgeStyle::Straight),
            (SkillTreeNodeName::WhiteNode, SkillTreeNodeName::YellowNode, EdgeStyle::Straight),
            (SkillTreeNodeName::WhiteNode, SkillTreeNodeName::GreenNode, EdgeStyle::Straight),
            (
                SkillTreeNodeName::RedNode,
                SkillTreeNodeName::OrangeNode,
                EdgeStyle::Curved(Vec2::new(0.0, 0.0)),
            ),
            // (
            //     SkillTreeNodeName::OrangeNode,
            //     SkillTreeNodeName::YellowNode,
            //     EdgeStyle::Curved(Vec2::new(0.0, 0.0)),
            // ),
            // (
            //     SkillTreeNodeName::YellowNode,
            //     SkillTreeNodeName::GreenNode,
            //     EdgeStyle::Curved(Vec2::new(0.0, 0.0)),
            // ),
            // (
            //     SkillTreeNodeName::GreenNode,
            //     SkillTreeNodeName::RedNode,
            //     EdgeStyle::Curved(Vec2::new(0.0, 0.0)),
            // ),
        ],
    }
}

#[derive(Resource)]
pub struct DynamicImages {
    pub circles: HashMap<(u32, u32), Handle<Image>>,
}

impl Default for DynamicImages {
    fn default() -> Self {
        DynamicImages { circles: HashMap::new() }
    }
}

const TEXTURE_BOARDER: u32 = 2;

impl DynamicImages {
    pub fn get_circle(&mut self, radius: UVec2, thickness: f32, assets: &mut Assets<Image>) -> Handle<Image> {
        if let Some(handle) = self.circles.get(&(radius.x, radius.y)) {
            return handle.clone();
        }

        let size = UVec2::new(
            (radius.x + TEXTURE_BOARDER + thickness as u32) * 2,
            (radius.y + TEXTURE_BOARDER + thickness as u32) * 2,
        );

        let image = Image::new(
            Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            ellipse_image(size, UVec2::new(radius.x, radius.y), thickness / 2.0),
            TextureFormat::Rgba8Unorm,
            bevy::asset::RenderAssetUsages::MAIN_WORLD | bevy::asset::RenderAssetUsages::RENDER_WORLD,
        );
        let handle = assets.add(image);
        self.circles.insert((radius.x, radius.y), handle.clone());
        handle
    }
}

fn ellipse_image(size: UVec2, radius: UVec2, thickness: f32) -> Vec<u8> {
    let mut data = vec![255 as u8; (size.x * size.y * 4) as usize];
    let center = size.as_vec2() / 2.0;
    let exterior_radius = radius.as_vec2() + thickness;
    let interior_radius = radius.as_vec2() - thickness;

    for y in 0..(center.y.ceil() as u32) {
        for x in 0..(center.x.ceil() as u32) {
            let point = Vec2::new(x as f32, y as f32) - center;
            let outer_dist = (point.x / exterior_radius.x).powf(2.0) + (point.y / exterior_radius.y).powf(2.0);
            let inner_dist = (point.x / interior_radius.x).powf(2.0) + (point.y / interior_radius.y).powf(2.0);

            let alpha = if outer_dist < 1.0 && inner_dist > 1.0 { 255 } else { 0 };

            // Assign to the four corners.
            let px = x * 4;
            let nx = (size.x - x - 1) * 4;
            let py = y * size.x * 4;
            let ny = (size.y - y - 1) * size.x * 4;
            data[(py + px + 3) as usize] = alpha;
            data[(py + nx + 3) as usize] = alpha;
            data[(ny + px + 3) as usize] = alpha;
            data[(ny + nx + 3) as usize] = alpha;
        }
    }
    data
}
