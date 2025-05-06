use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct Button<T: Event + Default> {
    trigger: T,
    text: String,
    width: Val,
    height: Val,
}

impl<T: Event + Default> Default for Button<T> {
    fn default() -> Self {
        Button {
            trigger: T::default(),
            text: "Button".to_string(),
            width: Val::Px(150.0),
            height: Val::Px(65.0),
        }
    }
}

impl<T: Event + Default> Button<T> {
    pub fn new(text: String) -> Self {
        Button {
            text: text,
            ..default()
        }
    }

    fn system(
        mut commands: Commands,
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &Children),
            (Changed<Interaction>, With<bevy::prelude::Button>),
        >,
    ) {
        for (interaction, mut color, children) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    *color = PRESSED_BUTTON.into();
                    commands.trigger(T::default());
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    **text = "Button".to_string();
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }

    pub fn spawn(&self, parent: &mut ChildBuilder, asset_server: Res<AssetServer>) {
        parent
            .spawn((
                Button,
                Node {
                    width: self.width,
                    height: self.height,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child((
                Text::new(self.text.clone()),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
    }
}
