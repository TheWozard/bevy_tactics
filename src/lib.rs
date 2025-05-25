use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod game;
pub mod ui;

pub fn baseline_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "SimpleGame".to_string(),
            ..default()
        }),
        ..default()
    }));

    #[cfg(debug_assertions)]
    {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        });
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F1)),
        );
    }

    app.add_systems(Startup, camera_setup);
    app.add_systems(Update, exit_condition);

    app
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn exit_condition(
    mut app_exit_events: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
