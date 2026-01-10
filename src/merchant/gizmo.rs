use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, gizmo_shop_time);
}

fn gizmo_shop_time(mut gizmos: Gizmos, shop_open_timer: Res<crate::merchant::ShopOpenTimer>) {
    if shop_open_timer.is_open() {
        let ratio = shop_open_timer.fraction();

        let height = 100.0;
        let base = Vec2::new(-400.0, -height / 2.0);

        if ratio > 0.0 {
            gizmos.line_2d(
                base,
                base + Vec2::new(0.0, height * ratio),
                Color::hsv(127.0, 255.0, 1.0),
            );
        }
        if ratio < 1.0 {
            gizmos.line_2d(
                base + Vec2::new(0.0, height * ratio),
                base + Vec2::new(0.0, height),
                Color::hsva(127.0, 0.0, 1.0, 0.2),
            );
        }
    }
}
