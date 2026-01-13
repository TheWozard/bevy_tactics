use bevy::input::common_conditions::input_just_pressed;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod game;
pub mod merchant;
pub mod random;
pub mod skilltree;
pub mod theme;
pub mod ui;
pub mod util;

pub fn baseline_app() -> App {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SimpleGame".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    #[cfg(debug_assertions)]
    {
        app.add_plugins(EguiPlugin { ..default() });
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F1)),
        );
    }

    app.add_systems(Startup, camera_setup);
    app.add_systems(
        Update,
        exit_condition.run_if(input_just_pressed(KeyCode::Escape)),
    );
    app.add_plugins(random::plugin);
    app.add_plugins(theme::plugin);

    app
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn exit_condition(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.write(AppExit::Success);
}
