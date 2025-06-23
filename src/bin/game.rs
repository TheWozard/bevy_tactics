fn main() {
    let mut app = bevy_learning::baseline_app();
    app.add_plugins(bevy_learning::game::plugin);
    app.run();
}
