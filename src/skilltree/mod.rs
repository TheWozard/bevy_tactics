use std::{collections::HashMap, f32::consts::PI};

use bevy::prelude::*;

pub fn plugin(app: &mut bevy::prelude::App) {
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

pub fn spawn_skill_tree(
    mut events: EventReader<SkillTreeSpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

            commands.spawn((
                Transform::from_translation(((start_node_position + end_node_position) / 2.0).extend(1.0))
                    .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x)))
                    .with_scale(Vec3::new(direction.length(), UI_EDGE_SCALE, 1.0)),
                Mesh2d(edge_mesh.clone()),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
                Name::new("SkillTreeStraightEdge"),
            ));
        }
        for node in &event.tree.nodes {
            commands.spawn((
                Transform::from_translation((node.1.position * UI_NODE_SPACING).extend(2.0))
                    .with_scale(Vec3::new(UI_NODE_SCALE, UI_NODE_SCALE, 1.0))
                    .with_rotation(Quat::from_rotation_z(PI / 4.0)),
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

pub struct Tree<T> {
    pub nodes: HashMap<SkillTreeNodeName, T>,
    pub edges: Vec<(SkillTreeNodeName, SkillTreeNodeName)>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum SkillTreeNodeName {
    Invalid,

    Root,
    Value1,
    Value2,
    Value3,
    BranchA1,
    BranchA2,
    BranchB1,
    BranchB2,
}

pub struct SkillTreeNode {
    pub position: Vec2,
    pub color: Color,
}

pub fn example_skill_tree() -> Tree<SkillTreeNode> {
    Tree {
        nodes: HashMap::from([
            (
                SkillTreeNodeName::Root,
                SkillTreeNode {
                    position: Vec2::new(0.0, 1.0),
                    color: Color::hsv(0.0, 0.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::Value1,
                SkillTreeNode {
                    position: Vec2::new(1.0, 2.0),
                    color: Color::hsv(0.0, 0.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::Value2,
                SkillTreeNode {
                    position: Vec2::new(-1.0, 2.0),
                    color: Color::hsv(0.0, 0.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::Value3,
                SkillTreeNode {
                    position: Vec2::new(0.0, 3.0),
                    color: Color::hsv(0.0, 0.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::BranchA1,
                SkillTreeNode {
                    position: Vec2::new(2.0, 3.0),
                    color: Color::hsv(0.0, 255.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::BranchA2,
                SkillTreeNode {
                    position: Vec2::new(3.0, 2.0),
                    color: Color::hsv(0.0, 255.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::BranchB1,
                SkillTreeNode {
                    position: Vec2::new(-2.0, 3.0),
                    color: Color::hsv(127.0, 255.0, 1.0),
                },
            ),
            (
                SkillTreeNodeName::BranchB2,
                SkillTreeNode {
                    position: Vec2::new(-3.0, 2.0),
                    color: Color::hsv(127.0, 255.0, 1.0),
                },
            ),
        ]),
        edges: vec![
            (SkillTreeNodeName::Root, SkillTreeNodeName::Value1),
            (SkillTreeNodeName::Value1, SkillTreeNodeName::Value2),
            (SkillTreeNodeName::Value2, SkillTreeNodeName::Value3),
            (SkillTreeNodeName::Value3, SkillTreeNodeName::BranchA1),
            (SkillTreeNodeName::Value3, SkillTreeNodeName::BranchB1),
            (SkillTreeNodeName::BranchA1, SkillTreeNodeName::BranchA2),
            (SkillTreeNodeName::BranchB1, SkillTreeNodeName::BranchB2),
        ],
    }
}
