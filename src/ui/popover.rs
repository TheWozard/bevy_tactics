use bevy::prelude::*;
use bevy::render::camera;
use bevy::ui::*;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Update, Popover::state_change);
    app.add_systems(
        Update,
        (PopoverContent::mark, PopoverContent::sweep).chain(),
    );
    app.add_systems(PreUpdate, KeepNodeInWindow::system);
    app.add_systems(
        PreUpdate,
        (
            IntentionalInteractionTimer::update,
            IntentionalInteractionTimer::tick,
        )
            .chain()
            .after(ui_focus_system),
    );
}

// PopoverPosition defines the position of the popover relative to the hovered content.
#[derive(Clone, Copy, Debug)]
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

impl Default for Position {
    fn default() -> Self {
        Position::Top
    }
}

// Popover is a component that defines a popover UI element. Popovers can be positioned relative to other UI elements.
// Popover has a built in delay before it is shown, to prevent flickering when the moving the mouse quickly over the UI.
#[derive(Component, Debug)]
#[require(Interaction, IntentionalInteraction::None)]
pub struct Popover {
    position: Position,
    spawn: fn(&mut Commands, &AssetServer, &Details) -> Entity,
    spawned: Option<Entity>,
    depth: i32,
}

impl Popover {
    fn new(position: Position, spawn: fn(&mut Commands, &AssetServer, &Details) -> Entity) -> Self {
        Self {
            position,
            spawn,
            spawned: None,
            depth: 0,
        }
    }

    fn state_change(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut interaction_query: Query<
            (Entity, &mut Popover, &ComputedNode, &IntentionalInteraction),
            Changed<IntentionalInteraction>,
        >,
        mut child_query: Query<&Interaction>,
    ) {
        for (parent, mut popover, node, interaction) in &mut interaction_query {
            match *interaction {
                IntentionalInteraction::Hovered => {
                    if popover.spawned.is_none() {
                        let entity = (popover.spawn)(
                            &mut commands,
                            &assets,
                            &Details {
                                position: popover.position,
                                size: node.size / 2.,
                                index: node.stack_index as i32,
                                depth: popover.depth + 1,
                            },
                        );
                        popover.spawned = Some(entity);
                        commands.entity(parent).add_related::<ChildOf>(&[entity]);
                    }
                }
                IntentionalInteraction::None => {
                    if let Some(entity) = popover.spawned {
                        if let Ok(interaction) = child_query.get_mut(entity) {
                            if interaction == &Interaction::None {
                                popover.despawn(&mut commands);
                            }
                        }
                    }
                }
            }
        }
    }

    fn despawn(&mut self, commands: &mut Commands) {
        if let Some(entity) = self.spawned {
            commands.entity(entity).try_despawn();
            self.spawned = None;
        }
    }
}

// HoverPosition defines details about where the hover content should be placed.
// Provides a bundle that can be used to position the content.
#[derive(Default)]
pub struct Details {
    position: Position,
    size: Vec2,
    index: i32,
    depth: i32,
}

impl Details {
    pub fn bundle(&self) -> impl Bundle {
        (
            self.node(),
            PopoverContent::default(),
            FocusPolicy::Block,
            ZIndex(self.index + 1),
            GlobalZIndex(1), // TODO: This should become a constant.
            KeepNodeInWindow,
            Visibility::Hidden,
        )
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }

    pub fn popover(
        &self,
        position: Position,
        spawn: fn(&mut Commands, &AssetServer, &Details) -> Entity,
    ) -> Popover {
        Popover {
            depth: self.depth,
            ..Popover::new(position, spawn)
        }
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
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }
}

// PopoverContent is a component that handles cleanup of popover contents.
#[derive(Component, Default, Debug)]
#[require(Interaction::None, IntentionalInteraction::None)]
struct PopoverContent {
    ignore: bool,
}

impl PopoverContent {
    pub fn sweep(
        mut commands: Commands,
        mut query: Query<
            (&mut PopoverContent, &ChildOf, &IntentionalInteraction),
            (Changed<IntentionalInteraction>, With<PopoverContent>),
        >,
        mut parent_query: Query<(&ChildOf, Option<&mut Popover>, Option<&Interaction>)>,
    ) {
        for (mut cleanup, parent, interaction) in &mut query {
            match (cleanup.ignore, interaction) {
                (false, IntentionalInteraction::None) => {
                    // Cleanup parents.
                    let mut next = parent.0;
                    // Traverse the hierarchy to find the highest parent that is not protected or focused.
                    while let Ok((parent, popover, interaction)) = parent_query.get_mut(next) {
                        // Find the first parent that is being interacted with and stop there.
                        if let Some(interaction) = interaction {
                            if interaction != &Interaction::None {
                                break;
                            }
                        }
                        // If we pass a popover, de-spawn its contents as nothing in the hierarchy is being interacted with.
                        if let Some(mut popover) = popover {
                            popover.despawn(&mut commands);
                        }
                        // Loop through parents.
                        next = parent.0;
                    }
                }
                (true, _) => {
                    cleanup.ignore = false;
                }
                (_, _) => {}
            }
        }
    }

    pub fn mark(
        mut query: Query<
            (Option<&ChildOf>, &Interaction),
            (Changed<Interaction>, With<PopoverContent>),
        >,
        mut parent_query: Query<(&ChildOf, Option<&mut PopoverContent>, Option<&Popover>)>,
    ) {
        for (parent, interaction) in &mut query {
            if parent.is_none() {
                continue;
            }
            let mut next = parent.unwrap().0;
            // Traverse the hierarchy to mark all unmarked parents as ignored.
            while let Ok((parent, cleanup, popover)) = parent_query.get_mut(next) {
                if let Some(mut cleanup) = cleanup {
                    cleanup.ignore = *interaction != Interaction::None;
                }
                if let Some(popover) = popover {
                    // To prevent traversing the whole hierarchy, we stop if we find a root popover.
                    if popover.depth == 0 {
                        break;
                    }
                }
                next = parent.0;
            }
        }
    }
}

// KeepNodeInWindow is a component that keeps a node within the bounds of its camera.
// This should be spawned with Visibility::Hidden to prevent flickering.
#[derive(Component)]
#[require(Node, Visibility::Hidden)]
pub struct KeepNodeInWindow;

impl KeepNodeInWindow {
    fn system(
        cameras: Query<&camera::Camera>,
        mut query: Query<
            (
                &mut Node,
                &GlobalTransform,
                &ComputedNode,
                &ComputedNodeTarget,
                Option<&mut Visibility>,
            ),
            (With<KeepNodeInWindow>, Changed<ComputedNode>),
        >,
    ) {
        for (mut node, global, computed, target, visibility) in &mut query {
            // Load Camera Details.
            let camera = cameras.get(target.camera().unwrap()).unwrap();
            let viewport = camera.physical_viewport_size().unwrap().as_vec2();

            // Calculate the amount of offset needed to keep the node in the viewport.
            let location = global.translation().truncate();
            let size = computed.size / 2.;
            let mut offset = Vec2::new(0., 0.);
            if location.x < size.x {
                offset.x = size.x - location.x;
            } else if location.x > viewport.x - size.x {
                offset.x = location.x - (viewport.x - size.x);
            }
            if location.y < size.y {
                offset.y = size.y - location.y;
            } else if location.y > viewport.y - size.y {
                offset.y = location.y - (viewport.y - size.y);
            }

            // Move the node if an offset is needed.
            if offset.x != 0. {
                match (node.left, node.right) {
                    (Val::Auto, Val::Auto) => {
                        if offset.x > 0. {
                            node.right = Val::Px(0.);
                        } else {
                            node.left = Val::Px(0.);
                        }
                    }
                    (_, _) => {
                        (node.left, node.right) = (node.right, node.left);
                    }
                }
            }
            if offset.y != 0. {
                match (node.top, node.bottom) {
                    (Val::Auto, Val::Auto) => {
                        if offset.y > 0. {
                            node.top = Val::Px(0.);
                        } else {
                            node.bottom = Val::Px(0.);
                        }
                    }
                    (_, _) => {
                        (node.top, node.bottom) = (node.bottom, node.top);
                    }
                }
            }

            // Node might be hidden to hide flickering.
            if let Some(mut visibility) = visibility {
                if *visibility == Visibility::Hidden {
                    *visibility = Visibility::Inherited;
                }
            }
        }
    }
}

// IntentionalInteraction is a component that indicates whether the user is intentionally interacting with the UI element.
// This is used to prevent flickering when the mouse is moved quickly over the UI. Or to make it easier to move from a small button to a popover.
#[derive(Component)]
#[require(IntentionalInteractionTimer, Interaction::None)]
pub enum IntentionalInteraction {
    None,
    Hovered,
}

impl IntentionalInteraction {
    pub fn invert(&mut self) {
        match self {
            IntentionalInteraction::None => *self = IntentionalInteraction::Hovered,
            IntentionalInteraction::Hovered => *self = IntentionalInteraction::None,
        }
    }
}

// IntentionalInteractionTimer is a component that provides a timer to control the intentional interaction state.
#[derive(Component)]
pub struct IntentionalInteractionTimer {
    timer: Timer,
}

impl Default for IntentionalInteractionTimer {
    fn default() -> Self {
        Self::new(0.15)
    }
}

impl IntentionalInteractionTimer {
    pub fn new(time: f32) -> Self {
        let mut timer = Timer::from_seconds(time, TimerMode::Once);
        timer.set_elapsed(timer.duration());
        Self { timer }
    }

    pub fn tick(
        mut query: Query<(
            &mut IntentionalInteractionTimer,
            &mut IntentionalInteraction,
        )>,
        time: Res<Time>,
    ) {
        for (mut timer, mut interaction) in &mut query {
            timer.timer.tick(time.delta());
            if timer.timer.just_finished() {
                interaction.invert();
            }
        }
    }

    pub fn update(
        mut query: Query<
            (
                &mut IntentionalInteractionTimer,
                &Interaction,
                &IntentionalInteraction,
            ),
            Changed<Interaction>,
        >,
    ) {
        for (mut timer, interaction, intentional) in &mut query {
            match (interaction, intentional) {
                (Interaction::Hovered, IntentionalInteraction::None)
                | (Interaction::Pressed, IntentionalInteraction::None)
                | (Interaction::None, IntentionalInteraction::Hovered) => {
                    timer.timer.unpause();
                    timer.timer.reset();
                }
                (_, _) => {
                    timer.timer.pause();
                }
            }
        }
    }
}
