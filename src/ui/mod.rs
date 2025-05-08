use bevy::prelude::*;

mod popover;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (ButtonState::state_change, ButtonState::tick).chain(),
    );
    app.add_plugins(popover::plugin);
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
            column_gap: Val::Px(10.0),
            ..default()
        },
        children![
            (
                Node {
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                children![
                    button(&assets, "Top", popover::Position::Top),
                    button(&assets, "TopLeft", popover::Position::TopLeft),
                    button(&assets, "TopRight", popover::Position::TopRight),
                    button(&assets, "Bottom", popover::Position::Bottom),
                    button(&assets, "BottomLeft", popover::Position::BottomLeft),
                    button(&assets, "BottomRight", popover::Position::BottomRight),
                ],
            ),
            (
                Node {
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                children![
                    button(&assets, "Right", popover::Position::Right),
                    button(&assets, "RightTop", popover::Position::RightTop),
                    button(&assets, "RightBottom", popover::Position::RightBottom),
                    button(&assets, "Left", popover::Position::Left),
                    button(&assets, "LeftTop", popover::Position::LeftTop),
                    button(&assets, "LeftBottom", popover::Position::LeftBottom),
                ],
            )
        ],
    ));
}

fn button(
    assets: &AssetServer,
    text: impl Into<String>,
    position: popover::Position,
) -> impl Bundle {
    (
        ButtonState::default(),
        Button,
        Node {
            min_width: Val::Px(250.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new(text),
            TextFont {
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
        popover::Popover::new(position, content_a),
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

fn content_a(commands: &mut Commands, assets: &AssetServer, hover: &popover::Details) -> Entity {
    content(commands, assets, hover, "Lorem ipsum dolor sit amet")
}

fn content(
    commands: &mut Commands,
    assets: &AssetServer,
    hover: &popover::Details,
    text: impl Into<String>,
) -> Entity {
    commands
        .spawn((
            hover.bundle(),
            BackgroundColor(HOVERED_BUTTON),
            children![(
                Text::new(text),
                TextFont {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )],
        ))
        .id()
}
