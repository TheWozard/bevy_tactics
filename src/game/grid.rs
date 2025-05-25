use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Grid>();
    app.register_type::<GridLocation>();
    app.register_type::<GridContent>();
    app.register_type::<GridOwner>();
    app.register_type::<GridOwned>();
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform, Name::new("Grid"))]
pub struct Grid {
    pub size: IVec2,
    pub data: Vec<GridContent>,
}

#[derive(Clone, Debug, Reflect)]
pub struct GridContent {
    pub tile: Entity,
}

impl Grid {
    pub fn new(size: IVec2) -> Self {
        Grid {
            size,
            data: Vec::with_capacity((size.x * size.y) as usize),
        }
    }

    pub fn get_size(&self) -> IVec2 {
        self.size
    }

    pub fn index(&self, location: IVec2) -> Option<usize> {
        if location.y < self.size.y
            && location.x < self.size.x
            && location.x >= 0
            && location.y >= 0
        {
            Some((location.y * self.size.x + location.x) as usize)
        } else {
            None
        }
    }

    pub fn get_location(&self, location: IVec2) -> Option<GridLocation> {
        if self.index(location).is_some() {
            Some(GridLocation::new(location))
        } else {
            None
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform)]
pub struct GridLocation {
    location: IVec2,
}

impl GridLocation {
    fn new(location: IVec2) -> Self {
        GridLocation { location }
    }

    pub fn as_ivec2(&self) -> IVec2 {
        self.location
    }
}

impl From<&GridLocation> for IVec2 {
    fn from(location: &GridLocation) -> IVec2 {
        location.location
    }
}

#[derive(Component, Debug, Reflect)]
#[relationship(relationship_target = GridOwned)]
pub struct GridOwner(Entity);

#[derive(Component, Debug, Reflect)]
#[relationship_target(relationship = GridOwner)]
pub struct GridOwned(Vec<Entity>);
