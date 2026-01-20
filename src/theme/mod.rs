use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, Textures::load);
}

pub enum Anchor {
    Center,
    BottomCenter,
}

pub struct Texture {
    handle: Handle<Image>,
    size: Vec2,
    anchor: Anchor,
}

impl Texture {
    pub fn sprite(&self) -> Sprite {
        Sprite {
            image: self.handle.clone(),
            custom_size: Some(self.size),
            ..default()
        }
    }

    pub fn scale(&self) -> Vec2 {
        self.size
    }

    pub fn offset(&self) -> Vec2 {
        match self.anchor {
            Anchor::Center => self.size * -0.5,
            Anchor::BottomCenter => Vec2::new(self.size.x * 0.5, 0.0),
        }
    }

    pub fn translation(&self, position: &Vec3) -> Vec3 {
        position + self.offset().extend(0.0)
    }
}

#[derive(Resource)]
pub struct Textures {
    pub tile: Texture,
    pub unit: Texture,
    pub attack: Texture,
    pub swing: Texture,
}

impl Textures {
    fn load(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let scale = 16.0;
        commands.insert_resource(Textures {
            attack: Texture {
                handle: asset_server.load("tiles/attack.png"),
                size: Vec2::splat(scale / 2.0),
                anchor: Anchor::Center,
            },
            swing: Texture {
                handle: asset_server.load("tiles/swing.png"),
                size: Vec2::new(scale / 4.0, scale / 2.0),
                anchor: Anchor::Center,
            },
            tile: Texture {
                handle: asset_server.load("tiles/tile.png"),
                size: Vec2::splat(scale),
                anchor: Anchor::Center,
            },
            unit: Texture {
                handle: asset_server.load("tiles/unit.png"),
                size: Vec2::splat(scale),
                anchor: Anchor::Center,
            },
        });
    }
}
