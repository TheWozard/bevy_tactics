use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(Sprites::plugin);
}

const FONT_FILE: &str = "fonts/Nunito-Regular.ttf";
const FONT_SCALE: f32 = 24.0;

const BACKGROUND_COLOR: Color = Color::hsva(0.0, 0.0, 0.5, 1.0);
const TEXT_COLOR: Color = Color::hsva(0.0, 0.0, 0.0, 1.0);

const PADDING_UNIT: f32 = 10.0;
const RADIUS_UNIT: f32 = 5.0;

pub fn baseline() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexEnd,
            column_gap: Val::Px(PADDING_UNIT),
            padding: UiRect::all(Val::Px(PADDING_UNIT)),
            ..default()
        },
        Pickable::IGNORE,
    )
}

pub fn styled_span() -> impl Bundle {
    (
        Node {
            padding: UiRect::all(Val::Px(PADDING_UNIT)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        BorderRadius::all(Val::Px(RADIUS_UNIT)),
    )
}

pub fn text_style(assets: &AssetServer) -> impl Bundle {
    (
        TextFont {
            font: assets.load(FONT_FILE),
            font_size: FONT_SCALE,
            ..default()
        },
        TextColor(TEXT_COLOR),
    )
}

#[derive(Resource)]
pub struct Sprites {
    pub scale: f32,
    image: Handle<Image>,
    tile: Handle<Image>,
    unit: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl Sprites {
    pub fn plugin(app: &mut App) {
        app.add_systems(PreStartup, Self::load);
    }

    fn load(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        commands.insert_resource(Sprites {
            scale: 64.0,
            image: asset_server.load("images/64x64.png"),
            tile: asset_server.load("tiles/tile.png"),
            unit: asset_server.load("tiles/unit.png"),
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::splat(64),
                16,
                137,
                None,
                None,
            )),
        });
    }

    pub fn bundle(&self, index: usize) -> impl Bundle {
        Sprite {
            image: self.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: self.layout.clone(),
                index,
            }),
            ..default()
        }
    }

    pub fn tile_sprite(&self) -> Sprite {
        Sprite {
            image: self.tile.clone(),
            ..default()
        }
    }

    pub fn unit_bundle(&self) -> impl Bundle {
        Sprite {
            image: self.unit.clone(),
            ..default()
        }
    }
}

pub fn grid(rows: usize, cols: usize, center: Vec2, width: usize) -> impl Iterator<Item = Vec2> {
    let unit = (width / (cols - 1)) as f32;
    let corner = center
        - Vec2::new(
            (cols as f32 - 1.0) * 0.5 * unit,
            (rows as f32 - 1.0) * 0.5 * unit,
        );
    (0..rows).flat_map(move |y| {
        (0..cols).map(move |x| {
            let position = Vec2::new(x as f32, y as f32);
            (position * unit) + corner
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let positions: Vec<_> = grid(2, 5, Vec2::new(100.0, 0.0), 200).collect();
        assert_eq!(positions.len(), 10);
        assert_eq!(positions[0], Vec2::new(0.0, -25.0));
        assert_eq!(positions[1], Vec2::new(50.0, -25.0));
        assert_eq!(positions[2], Vec2::new(100.0, -25.0));
        assert_eq!(positions[3], Vec2::new(150.0, -25.0));
        assert_eq!(positions[4], Vec2::new(200.0, -25.0));
        assert_eq!(positions[5], Vec2::new(0.0, 25.0));
        assert_eq!(positions[6], Vec2::new(50.0, 25.0));
        assert_eq!(positions[7], Vec2::new(100.0, 25.0));
        assert_eq!(positions[8], Vec2::new(150.0, 25.0));
        assert_eq!(positions[9], Vec2::new(200.0, 25.0));
    }
}
