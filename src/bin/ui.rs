fn main() {
    let mut app = bevy_simple::baseline_app();
    app.add_plugins(bevy_simple::ui::plugin);
    app.run();
}
