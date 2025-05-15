use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_simple::ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "UI".to_string(),
            ..default()
        }),
        ..default()
    }));

    app.add_systems(Startup, camera_setup);
    app.add_systems(Update, exit_condition);

    app.add_plugins(ui::plugin);

    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    #[cfg(debug_assertions)] // debug/dev builds only
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(LogDiagnosticsPlugin::default());
    }

    app.run();
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
