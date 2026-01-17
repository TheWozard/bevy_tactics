fn main() {
    let mut app = bevy_tactics::baseline_app();
    app.add_plugins(bevy_tactics::game::plugin);
    app.run();
}
