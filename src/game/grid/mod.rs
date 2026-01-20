use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::util::*;

mod grid;
pub mod selection;

pub fn plugin(app: &mut App) {
    app.add_observer(on_remove_grid_location);

    app.register_type::<Grid>();
    app.register_type::<GridLocation>();
    app.register_type::<Space>();
    app.register_type::<GridOwner>();
    app.register_type::<GridOwned>();
}

#[derive(Clone, Debug, Reflect)]
pub struct Space {
    pub unit: Option<Entity>,
    pub tile: Option<Entity>,
}

impl Default for Space {
    fn default() -> Self {
        Space {
            unit: None,
            tile: None,
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform, Name::new("Grid"))]
// A 2D grid that stores entities in a fixed-size grid.
pub struct Grid {
    scale: IVec2,
    grid: grid::Grid<Space>,
}

impl Grid {
    pub fn new(size: IVec2, scale: IVec2) -> Self {
        Grid {
            scale,
            grid: grid::Grid::<Space>::new(size),
        }
    }

    pub fn get(&self, kind: &EntityKind, location: &IVec2) -> Option<Entity> {
        self.grid.get(location).and_then(|space| kind.get(space))
    }

    fn take(&mut self, kind: &EntityKind, location: &IVec2) -> Option<Entity> {
        kind.take_optional(self.grid.get_mut(location))
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        kind: &EntityKind,
        location: &IVec2,
        bundle: impl Bundle,
    ) -> Option<Entity> {
        if let Some(space) = self.grid.get_mut(location) {
            if kind.is_none(space) {
                let entity = commands.spawn((GridLocation::new(location.clone(), kind.clone()), bundle)).id();
                kind.set(space, entity);
                return Some(entity);
            }
        }
        None
    }

    pub fn move_to(&mut self, from: &mut GridLocation, to: &IVec2) -> Option<Entity> {
        if from.kind.is_none_optional(self.grid.get(to)) {
            if let Some(entity) = from.kind.take_optional(self.grid.get_mut(from.as_ivec2())) {
                from.kind.set(self.grid.get_mut(to).unwrap(), entity);
                from.location = to.clone();
                return Some(entity);
            }
        }
        None
    }

    pub fn find(
        &self,
        location: &IVec2,
        direction: &IVec2,
        selection: selection::Shape,
        predicate: impl Fn(Entity) -> bool,
    ) -> Option<IVec2> {
        let mut iter = self
            .grid
            .iter_breath_first(location, direction, selection)
            .skip(1);
        while let Some(location) = self.grid.find(&mut iter, |v| v.is_some()) {
            if let Some(entity) = self.grid.get(&location) {
                if predicate(entity.clone()) {
                    return Some(location);
                }
            }
        }
        None
    }

    pub fn a_star_move(
        &mut self,
        from: &IVec2,
        to: &IVec2,
        steps: usize,
    ) -> Option<(GridLocation, Vec<IVec2>)> {
        let path = self.grid.a_star(from, to, |v| v.is_none());
        let mut trimmed_path = if path.len() > steps + 1 {
            path[..=steps].to_vec()
        } else {
            path[..].to_vec()
        };
        let mut end = trimmed_path.last().cloned()?;
        if end == *to {
            trimmed_path.pop();
            end = trimmed_path.last().cloned()?;
        }
        if let Some(target) = self.move_to(from, &end) {
            return Some((target, trimmed_path));
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Reflect)]
// Represents the type of entity that can occupy a space in the grid.
pub enum EntityKind {
    Unit,
    Tile,
}

impl EntityKind {
    pub fn is_none(&self, space: &Space) -> bool {
        match self {
            EntityKind::Unit => space.unit.is_none(),
            EntityKind::Tile => space.tile.is_none(),
        }
    }

    pub fn is_none_optional(&self, space: Option<&Space>) -> bool {
        match space {
            Some(space) => self.is_none(space),
            None => true,
        }
    }

    pub fn set(&self, space: &mut Space, entity: Entity) {
        match self {
            EntityKind::Unit => space.unit = Some(entity),
            EntityKind::Tile => space.tile = Some(entity),
        }
    }

    pub fn get(&self, space: &Space) -> Option<Entity> {
        match self {
            EntityKind::Unit => space.unit,
            EntityKind::Tile => space.tile,
        }
    }

    pub fn take(&self, space: &mut Space) -> Option<Entity> {
        match self {
            EntityKind::Unit => space.unit.take(),
            EntityKind::Tile => space.tile.take(),
        }
    }

    pub fn take_optional(&self, space: Option<&mut Space>) -> Option<Entity> {
        match space {
            Some(space) => self.take(space),
            None => None,
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform)]
pub struct GridLocation {
    location: IVec2,
    kind: EntityKind,
}

fn on_remove_grid_location(
    trigger: On<Remove, GridLocation>,
    location_query: Query<(&GridLocation, &GridOwner)>,
    mut grid_query: Query<&mut Grid>,
) {
    if let Ok((grid_location, grid_owned)) = location_query.get(trigger.event_target()) {
        if let Ok(mut grid) = grid_query.get_mut(grid_owned.get()) {
            grid.take(&grid_location.into(), &grid_location.into());
        }
    }
}

impl GridLocation {
    fn new(location: IVec2, kind: EntityKind) -> Self {
        GridLocation { location, kind }
    }
}

impl From<&GridLocation> for IVec2 {
    fn from(location: &GridLocation) -> IVec2 {
        location.location
    }
}

impl From<&GridLocation> for Vec2 {
    fn from(location: &GridLocation) -> Vec2 {
        location.location.as_vec2()
    }
}

impl From<&GridLocation> for EntityKind {
    fn from(location: &GridLocation) -> EntityKind {
        location.kind
    }
}

#[derive(Component, Debug, Reflect)]
#[relationship(relationship_target = GridOwned)]
pub struct GridOwner(Entity);

#[derive(Component, Debug, Reflect)]
#[relationship_target(relationship = GridOwner)]
pub struct GridOwned(Vec<Entity>);
