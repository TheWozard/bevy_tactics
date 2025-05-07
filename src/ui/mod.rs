use bevy::{prelude::*, scene::ron::de};

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (ButtonState::state_change, ButtonState::tick).chain(),
    );
    app.add_systems(Update, Popover::state_change);
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        children![
            button(&assets, "Button 1"),
            button(&assets, "Button 2"),
            button(&assets, "Button 3"),
        ],
    ));
}

fn button(assets: &AssetServer, text: impl Into<String>) -> impl Bundle {
    (
        ButtonState::default(),
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new(text),
            TextFont {
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
        Popover {
            spawn: content_a,
            spawned: None,
        },
    )
}

#[derive(PartialEq)]
pub enum ButtonStateEnum {
    Highlighted,
    Dimmed,
}

#[derive(Component)]
pub struct ButtonState {
    timer: Timer,
    state: ButtonStateEnum,
}

impl Default for ButtonState {
    fn default() -> Self {
        // We start in a finished state, so the timer has to be finished.
        let mut timer = Timer::from_seconds(0.2, TimerMode::Once);
        timer.set_elapsed(timer.duration());
        Self {
            timer: timer,
            state: ButtonStateEnum::Dimmed,
        }
    }
}

impl ButtonState {
    fn tick(mut timer_query: Query<(&mut ButtonState, &mut BackgroundColor)>, time: Res<Time>) {
        for (mut timer, mut color) in &mut timer_query {
            timer.timer.tick(time.delta());
            *color = timer.get_color().into();
        }
    }

    fn state_change(
        mut interaction_query: Query<
            (&Interaction, &mut ButtonState),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut state) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {}
                Interaction::Hovered => {
                    state.target_state(ButtonStateEnum::Highlighted);
                }
                Interaction::None => {
                    state.target_state(ButtonStateEnum::Dimmed);
                }
            }
        }
    }

    pub fn get_color(&self) -> Color {
        match self.state {
            ButtonStateEnum::Highlighted => {
                NORMAL_BUTTON.mix(&PRESSED_BUTTON, self.timer.fraction())
            }
            ButtonStateEnum::Dimmed => PRESSED_BUTTON.mix(&NORMAL_BUTTON, self.timer.fraction()),
        }
    }

    pub fn target_state(&mut self, target: ButtonStateEnum) {
        if self.state != target {
            self.state = target;
            let duration = self.timer.duration() - self.timer.elapsed();
            self.timer.reset();
            self.timer.set_elapsed(duration);
        }
    }
}

#[derive(Component)]
pub struct Popover {
    spawn: fn(&mut Commands, &AssetServer, &PopoverDetails) -> Entity,
    spawned: Option<Entity>,
}

impl Popover {
    pub fn state_change(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut interaction_query: Query<
            (
                &mut Popover,
                &Interaction,
                &ComputedNode,
                &GlobalTransform,
                &ZIndex,
            ),
            Changed<Interaction>,
        >,
    ) {
        for (mut popover, interaction, node, transform, index) in &mut interaction_query {
            match *interaction {
                Interaction::Hovered | Interaction::Pressed => {
                    if popover.spawned.is_none() {
                        let entity = (popover.spawn)(
                            &mut commands,
                            &assets,
                            &PopoverDetails {
                                translation: transform.translation().truncate() / 2.,
                                size: node.size / 4.,
                                index: index.0,
                                ..default()
                            },
                        );
                        popover.spawned = Some(entity);
                    }
                }
                Interaction::None => {
                    if let Some(spawned) = popover.spawned {
                        commands.entity(spawned).try_despawn();
                        popover.spawned = None;
                    }
                }
            }
        }
    }
}

fn content_a(commands: &mut Commands, assets: &AssetServer, hover: &PopoverDetails) -> Entity {
    content(commands, assets, hover, "Content A")
}

fn content(
    commands: &mut Commands,
    assets: &AssetServer,
    hover: &PopoverDetails,
    text: impl Into<String>,
) -> Entity {
    commands
        .spawn((
            hover.root(),
            children![(
                hover.bundle(),
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new(text),
                    TextFont {
                        font: assets.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            )],
        ))
        .id()
}

pub enum PopoverPosition {
    Top,
    Bottom,
}

// HoverPosition defines details about where the hover content should be placed.
// Provides a bundle that can be used to position the content.
pub struct PopoverDetails {
    position: PopoverPosition,
    translation: Vec2,
    size: Vec2,
    index: i32,
}

impl Default for PopoverDetails {
    fn default() -> Self {
        Self {
            position: PopoverPosition::Top,
            translation: Vec2::ZERO,
            size: Vec2::ZERO,
            index: 0,
        }
    }
}

impl PopoverDetails {
    pub fn bundle(&self) -> impl Bundle {
        (
            self.node(),
            ZIndex(self.index + 1),
            GlobalZIndex(1), // TODO: This should become a constant.
        )
    }

    pub fn root(&self) -> impl Bundle {
        let (justify_content, align_items) = self.orientation();
        (
            Node {
                display: Display::Flex,
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                justify_content,
                align_items,
                ..default()
            },
            GlobalZIndex(1), // TODO: This should become a constant.
        )
    }

    fn node(&self) -> Node {
        match self.position {
            PopoverPosition::Bottom => Node {
                // top: Val::Px(self.translation.y + self.size.y),
                // left: Val::Px(self.translation.x - self.size.x),
                padding: UiRect::all(Val::Px(10.0)), // TODO: This should be a constant.
                ..default()
            },
            PopoverPosition::Top => Node {
                bottom: Val::Px(self.translation.y + self.size.y),
                padding: UiRect::all(Val::Px(10.0)), // TODO: This should be a constant.
                ..default()
            },
        }
    }

    fn orientation(&self) -> (JustifyContent, AlignItems) {
        match self.position {
            PopoverPosition::Bottom => (JustifyContent::FlexStart, AlignItems::FlexStart),
            PopoverPosition::Top => (JustifyContent::FlexStart, AlignItems::FlexEnd),
        }
    }
}
