use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use rand::prelude::*;

mod item;
mod material;
mod popover;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (ButtonState::state_change, ButtonState::tick).chain(),
    );
    app.add_plugins(popover::plugin);
    app.add_plugins(item::plugin);
    app.add_plugins(material::plugin);
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

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
            item::Item {
                material: material::Material {
                    material: material::Type::None,
                    texture: assets.load("images/weapon_firesword.png"),
                    ..default()
                },
            }
            .bundle(),
            item::Item {
                material: material::Material {
                    material: material::Type::Glint,
                    texture: assets.load("images/weapon_sword.png"),
                    specular_map: Some(assets.load("images/weapon_sword_specular_map.png")),
                    ..default()
                },
            }
            .bundle(),
            item::Item {
                material: material::Material {
                    material: material::Type::Distortion,
                    direction: Vec2::new(-1.0, 1.0),
                    texture: assets.load("images/weapon_firesword.png"),
                    specular_map: Some(assets.load("images/weapon_firesword_distortion_map.png")),
                    ..default()
                },
            }
            .bundle(),
            item::Item {
                material: material::Material {
                    material: material::Type::Distortion,
                    direction: Vec2::new(0.0, 1.0),
                    texture: assets.load("images/weapon_potion.png"),
                    specular_map: Some(assets.load("images/weapon_potion_distortion_map.png")),
                    ..default()
                },
            }
            .bundle(),
        ],
    ));
}

const SIZE: Val = Val::Px(32.0);
const PADDING: UiRect = UiRect::horizontal(Val::Px(10.0));

const FONT_FILE: &str = "fonts/Nunito-Regular.ttf";
const FONT_SCALE: f32 = 32.0;
const FONT_COLOR: Color = Color::WHITE;

fn button(assets: &AssetServer, position: popover::Position) -> impl Bundle {
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
        BorderRadius::all(Val::Px(5.0)),
        BackgroundColor(NORMAL_BUTTON),
        children![
            (
                Text::new("Example"),
                TextFont {
                    font: assets.load(FONT_FILE),
                    font_size: FONT_SCALE,
                    ..default()
                },
                Node {
                    padding: PADDING,
                    ..default()
                },
                TextColor(FONT_COLOR),
            ),
            (
                ImageNode::new(assets.load("images/icon_diamond.png")).with_color(randomColor()),
                Node {
                    width: SIZE,
                    height: SIZE,
                    padding: PADDING,
                    ..default()
                },
            ),
            (
                ImageNode::new(assets.load("images/icon_fire.png")).with_color(randomColor()),
                Node {
                    width: SIZE,
                    height: SIZE,
                    padding: PADDING,
                    ..default()
                },
            ),
            (
                ImageNode::new(assets.load("images/icon_hourglass.png")).with_color(randomColor()),
                Node {
                    width: SIZE,
                    height: SIZE,
                    padding: PADDING,
                    ..default()
                },
            ),
            (
                ImageNode::new(assets.load("images/icon_shield.png")).with_color(randomColor()),
                Node {
                    width: SIZE,
                    height: SIZE,
                    padding: PADDING,
                    ..default()
                },
            ),
            (
                ImageNode::new(assets.load("images/icon_ice.png")).with_color(randomColor()),
                Node {
                    width: SIZE,
                    height: SIZE,
                    padding: PADDING,
                    ..default()
                },
            ),
        ],
        popover::Details::default().popover(position, content_a),
    )
}

fn randomColor() -> Color {
    let mut rng = rand::rng();
    Color::hsl(rng.random_range(0.0..360.0), 1., 0.6)
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
            BackgroundColor(color(hover.depth())),
            children![
                (
                    Node {
                        padding: UiRect::right(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![(
                        Text::new(text),
                        TextFont {
                            font: assets.load(FONT_FILE),
                            font_size: FONT_SCALE,
                            ..default()
                        },
                        TextColor(FONT_COLOR),
                    )],
                ),
                (
                    Button,
                    BackgroundColor(NORMAL_BUTTON),
                    hover.popover(hover.position(), content_a),
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    FocusPolicy::Pass,
                    children![(
                        Text::new("More"),
                        TextFont {
                            font: assets.load(FONT_FILE),
                            font_size: FONT_SCALE,
                            ..default()
                        },
                        TextColor(FONT_COLOR),
                    )],
                )
            ],
        ))
        .id()
}

fn color(index: i32) -> Color {
    match index % 2 {
        0 => Color::srgb(0.25, 0.25, 0.25),
        _ => Color::srgb(0.35, 0.35, 0.35),
    }
}
