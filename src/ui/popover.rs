use bevy::prelude::*;
use bevy::ui::FocusPolicy;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Update, (Popover::state_change, Popover::tick).chain());
    app.add_systems(Update, Cleanup::cleanup);
}

// PopoverPosition defines the position of the popover relative to the hovered content.
#[derive(Clone, Copy)]
pub enum Position {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Right,
    RightTop,
    RightBottom,
    Left,
    LeftTop,
    LeftBottom,
}

// Popover is a component that defines a popover UI element. Popovers can be positioned relative to other UI elements.
// Popover has a built in delay before it is shown, to prevent flickering when the moving the mouse quickly over the UI.
#[derive(Component)]
#[require(Interaction)]
pub struct Popover {
    position: Position,
    spawn: fn(&mut Commands, &AssetServer, &Details) -> Entity,
    spawned: Option<Entity>,
    delay: Timer,
}

impl Popover {
    pub fn new(
        position: Position,
        spawn: fn(&mut Commands, &AssetServer, &Details) -> Entity,
    ) -> Self {
        Self {
            position,
            spawn,
            spawned: None,
            delay: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }

    fn tick(
        mut commands: Commands,
        assets: Res<AssetServer>,
        time: Res<Time>,
        mut popover: Query<(Entity, &mut Popover, &ComputedNode, &ZIndex)>,
    ) {
        for (parent, mut popover, node, index) in &mut popover {
            popover.delay.tick(time.delta());
            if popover.delay.just_finished() && popover.spawned.is_none() {
                let entity = (popover.spawn)(
                    &mut commands,
                    &assets,
                    &Details {
                        position: popover.position,
                        size: node.size / 2.,
                        index: index.0,
                    },
                );
                popover.spawned = Some(entity);
                commands.entity(parent).add_related::<ChildOf>(&[entity]);
            }
        }
    }

    fn state_change(
        mut commands: Commands,
        mut interaction_query: Query<(&mut Popover, &Interaction), Changed<Interaction>>,
        mut child_query: Query<(&mut Cleanup, &Interaction)>,
    ) {
        for (mut popover, interaction) in &mut interaction_query {
            match *interaction {
                Interaction::Hovered | Interaction::Pressed => {
                    popover.delay.unpause();
                }
                Interaction::None => {
                    popover.delay.pause();
                    popover.delay.reset();
                    if let Some(entity) = popover.spawned {
                        if let Ok((mut cleanup, interaction)) = child_query.get_mut(entity) {
                            cleanup.protected = false;
                            if interaction == &Interaction::None {
                                commands.entity(entity).try_despawn();
                            }
                        }
                        popover.spawned = None;
                    }
                }
            }
        }
    }
}

// HoverPosition defines details about where the hover content should be placed.
// Provides a bundle that can be used to position the content.
pub struct Details {
    position: Position,
    size: Vec2,
    index: i32,
}

impl Details {
    pub fn bundle(&self) -> impl Bundle {
        (
            self.node(),
            Cleanup::default(),
            FocusPolicy::Block,
            ZIndex(self.index + 1),
            GlobalZIndex(1), // TODO: This should become a constant.
        )
    }

    fn node(&self) -> Node {
        match self.position {
            Position::Top => Node {
                bottom: Val::Px(self.size.y),
                ..self.baseline_node()
            },
            Position::TopLeft => Node {
                bottom: Val::Px(self.size.y),
                right: Val::ZERO,
                ..self.baseline_node()
            },
            Position::TopRight => Node {
                bottom: Val::Px(self.size.y),
                left: Val::ZERO,
                ..self.baseline_node()
            },
            Position::Bottom => Node {
                top: Val::Px(self.size.y),
                ..self.baseline_node()
            },
            Position::BottomLeft => Node {
                top: Val::Px(self.size.y),
                right: Val::ZERO,
                ..self.baseline_node()
            },
            Position::BottomRight => Node {
                top: Val::Px(self.size.y),
                left: Val::ZERO,
                ..self.baseline_node()
            },
            Position::Right => Node {
                left: Val::Px(self.size.x),
                ..self.baseline_node()
            },
            Position::RightTop => Node {
                left: Val::Px(self.size.x),
                top: Val::ZERO,
                ..self.baseline_node()
            },
            Position::RightBottom => Node {
                left: Val::Px(self.size.x),
                bottom: Val::ZERO,
                ..self.baseline_node()
            },
            Position::Left => Node {
                right: Val::Px(self.size.x),
                ..self.baseline_node()
            },
            Position::LeftTop => Node {
                right: Val::Px(self.size.x),
                top: Val::ZERO,
                ..self.baseline_node()
            },
            Position::LeftBottom => Node {
                right: Val::Px(self.size.x),
                bottom: Val::ZERO,
                ..self.baseline_node()
            },
        }
    }

    fn baseline_node(&self) -> Node {
        Node {
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Px(10.0)), // TODO: This should be a constant.
            ..default()
        }
    }
}

#[derive(Component)]
#[require(Interaction)]
struct Cleanup {
    protected: bool,
}

impl Default for Cleanup {
    fn default() -> Self {
        Self { protected: true }
    }
}

impl Cleanup {
    pub fn cleanup(
        mut commands: Commands,
        mut query: Query<(Entity, &Cleanup, &ChildOf, &Interaction), Changed<Interaction>>,
    ) {
        let mut traversal_query = query.clone();
        for (entity, cleanup, parent, interaction) in &mut query {
            if !cleanup.protected && interaction == &Interaction::None {
                let mut remove = entity;
                let mut next = parent.0;
                // Traverse the hierarchy to find the highest parent that is not protected or focused.
                loop {
                    if let Ok((entity, cleanup, parent, interaction)) =
                        traversal_query.get_mut(next)
                    {
                        if cleanup.protected || interaction != &Interaction::None {
                            break;
                        }
                        remove = entity;
                        next = parent.0;
                    } else {
                        break;
                    }
                }
                // De-spawn the popover and all its children.
                commands.entity(remove).despawn();
            }
        }
    }
}
