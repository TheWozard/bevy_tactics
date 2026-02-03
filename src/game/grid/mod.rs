use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

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
    grid: grid::Grid<Space>,
}

impl Grid {
    pub fn new(size: IVec2) -> Self {
        Grid {
            grid: grid::Grid::<Space>::new(size),
        }
    }

    pub fn size(&self) -> IVec2 {
        self.grid.size()
    }

    pub fn spaces(&self) -> i32 {
        self.grid.size().x * self.grid.size().y
    }

    // Gets an entity of a specific kind at the given location.
    pub fn get_entity(&self, kind: &EntityKind, location: &IVec2) -> Option<Entity> {
        self.grid.get(location).and_then(|space| kind.get(space))
    }

    // Sets an entity of a specific kind at the given location.
    fn set_entity(&mut self, kind: &EntityKind, location: &IVec2, entity: Entity) {
        if let Some(space) = self.grid.get_mut(location) {
            kind.set(space, entity);
        }
    }

    // Takes an entity of a specific kind from the given location if it exists. Returns the taken entity if successful.
    fn take_entity(&mut self, kind: &EntityKind, location: &IVec2) -> Option<Entity> {
        self.grid
            .get_mut(location)
            .and_then(|space| kind.take(space))
    }

    // Spawns an entity of a specific kind at the given location if the space is empty. Returns the spawned entity if successful.
    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        kind: &EntityKind,
        location: &IVec2,
        grid_entity: Entity,
        bundle: impl Bundle,
    ) -> Option<Entity> {
        if let Some(space) = self.grid.get_mut(location) {
            if kind.get(space).is_none() {
                let entity = commands
                    .spawn((
                        GridLocation::new(location.clone(), kind.clone()),
                        GridOwner(grid_entity),
                        bundle,
                    ))
                    .id();
                kind.set(space, entity);
                return Some(entity);
            }
        }
        None
    }

    // Moves an entity of a specific kind from one location to another if the target location is empty. Returns the moved entity if successful.
    pub fn move_to(&mut self, from: &mut GridLocation, to: &IVec2) -> Option<Entity> {
        if self.get_entity(&from.kind, to).is_none() {
            if let Some(entity) = self.take_entity(&from.kind, &from.location) {
                self.set_entity(&from.kind, to, entity);
                from.location = to.clone();
                return Some(entity);
            }
        }
        None
    }

    // finds the nearest entity of a specific kind from a starting location in a given direction and selection shape that satisfies a predicate.
    pub fn nearest_entity(
        &self,
        kind: &EntityKind,
        location: &IVec2,
        direction: &IVec2,
        selection: selection::Shape,
        predicate: impl Fn(&Entity) -> bool,
    ) -> Option<IVec2> {
        self.grid
            .iter_breath(location.clone(), direction.clone(), selection)
            .skip(1)
            .find(|location| {
                if let Some(entity) = self.get_entity(kind, location) {
                    return predicate(&entity);
                }
                return false;
            })
    }

    // finds the nearest empty location for a specific entity kind from a starting location in a given direction and selection shape.
    pub fn nearest_empty(
        &self,
        kind: &EntityKind,
        location: &IVec2,
        direction: &IVec2,
        selection: selection::Shape,
    ) -> Option<IVec2> {
        self.grid
            .iter_breath(location.clone(), direction.clone(), selection)
            .skip(1)
            .find(|location| self.get_entity(kind, location).is_none())
    }

    // A* pathfinding algorithm to find a path from start to end for a specific entity kind.
    pub fn a_star_to(
        &self,
        kind: &EntityKind,
        from: &IVec2,
        to: &IVec2,
        steps: usize,
    ) -> Vec<IVec2> {
        self.grid
            .a_star(from, to, |space| kind.get(space).is_none())
            .into_iter()
            .take(steps + 1)
            .collect()
    }

    // A* pathfinding algorithm to find a path from start to end, stopping next to the target.
    pub fn a_star_next_to(
        &self,
        kind: &EntityKind,
        from: &IVec2,
        to: &IVec2,
        steps: usize,
    ) -> Vec<IVec2> {
        let mut path = self.a_star_to(kind, from, to, steps);
        if let Some(last) = path.last() {
            if last == to {
                path.pop();
            }
        }
        path
    }
}

#[derive(Clone, Copy, Debug, Reflect)]
// Represents the type of entity that can occupy a space in the grid.
pub enum EntityKind {
    Unit,
    Tile,
}

impl EntityKind {
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
            grid.take_entity(grid_location.kind(), grid_location.location());
        }
    }
}

impl GridLocation {
    fn new(location: IVec2, kind: EntityKind) -> Self {
        GridLocation { location, kind }
    }

    pub fn location(&self) -> &IVec2 {
        &self.location
    }

    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform)]
pub struct GridScale {
    scale: IVec2,
}

impl GridScale {
    pub fn new(scale: IVec2) -> Self {
        GridScale { scale }
    }

    pub fn scale(&self) -> &IVec2 {
        &self.scale
    }

    pub fn iter_in_scale(
        &self,
        origin: Vec2,
        iter: impl Iterator<Item = IVec2>,
    ) -> impl Iterator<Item = Vec2> {
        let scale = self.scale.clone();
        iter.map(move |location| (location * scale).as_vec2() + origin)
    }
}

#[derive(Component, Debug, Reflect)]
#[relationship(relationship_target = GridOwned)]
pub struct GridOwner(Entity);

#[derive(Component, Debug, Reflect)]
#[relationship_target(relationship = GridOwner)]
pub struct GridOwned(Vec<Entity>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_star_to_empty_grid() {
        let grid = Grid::new(IVec2::new(5, 5));
        let path = grid.a_star_to(
            &EntityKind::Unit,
            &IVec2::new(0, 0),
            &IVec2::new(4, 4),
            100,
        );

        assert!(!path.is_empty());
        assert_eq!(path.first(), Some(&IVec2::new(0, 0)));
        assert_eq!(path.last(), Some(&IVec2::new(4, 4)));
        // Manhattan distance is 8, so path length should be 9
        assert_eq!(path.len(), 9);
    }

    #[test]
    fn test_a_star_to_with_step_limit() {
        let grid = Grid::new(IVec2::new(5, 5));
        let path = grid.a_star_to(&EntityKind::Unit, &IVec2::new(0, 0), &IVec2::new(4, 4), 3);

        // steps + 1 = 4 positions (start + 3 moves)
        assert_eq!(path.len(), 4);
        assert_eq!(path.first(), Some(&IVec2::new(0, 0)));
    }

    #[test]
    fn test_a_star_to_same_location() {
        let grid = Grid::new(IVec2::new(5, 5));
        let path = grid.a_star_to(
            &EntityKind::Unit,
            &IVec2::new(2, 2),
            &IVec2::new(2, 2),
            100,
        );

        assert_eq!(path.len(), 1);
        assert_eq!(path[0], IVec2::new(2, 2));
    }

    #[test]
    fn test_a_star_to_blocked_by_units() {
        let mut grid = Grid::new(IVec2::new(5, 5));
        let blocker = Entity::from_bits(1);

        // Block a vertical line at x=2
        for y in 0..5 {
            grid.set_entity(&EntityKind::Unit, &IVec2::new(2, y), blocker);
        }

        // Try to path from left to right - should fail (no path)
        let path = grid.a_star_to(
            &EntityKind::Unit,
            &IVec2::new(0, 2),
            &IVec2::new(4, 2),
            100,
        );

        assert!(path.is_empty());
    }

    #[test]
    fn test_a_star_to_tiles_dont_block_units() {
        let mut grid = Grid::new(IVec2::new(5, 5));
        let tile = Entity::from_bits(1);

        // Block with tiles at x=2
        for y in 0..5 {
            grid.set_entity(&EntityKind::Tile, &IVec2::new(2, y), tile);
        }

        // Units should still be able to path through tiles
        let path = grid.a_star_to(
            &EntityKind::Unit,
            &IVec2::new(0, 2),
            &IVec2::new(4, 2),
            100,
        );

        assert!(!path.is_empty());
        assert_eq!(path.first(), Some(&IVec2::new(0, 2)));
        assert_eq!(path.last(), Some(&IVec2::new(4, 2)));
    }

    #[test]
    fn test_a_star_to_units_dont_block_tiles() {
        let mut grid = Grid::new(IVec2::new(5, 5));
        let unit = Entity::from_bits(1);

        // Block with units at x=2
        for y in 0..5 {
            grid.set_entity(&EntityKind::Unit, &IVec2::new(2, y), unit);
        }

        // Tiles should still be able to path through units
        let path = grid.a_star_to(
            &EntityKind::Tile,
            &IVec2::new(0, 2),
            &IVec2::new(4, 2),
            100,
        );

        assert!(!path.is_empty());
        assert_eq!(path.first(), Some(&IVec2::new(0, 2)));
        assert_eq!(path.last(), Some(&IVec2::new(4, 2)));
    }

    #[test]
    fn test_a_star_to_path_around_obstacle() {
        let mut grid = Grid::new(IVec2::new(5, 5));
        let blocker = Entity::from_bits(1);

        // Create a partial wall at x=2, leaving a gap at y=0
        for y in 1..5 {
            grid.set_entity(&EntityKind::Unit, &IVec2::new(2, y), blocker);
        }

        let path = grid.a_star_to(
            &EntityKind::Unit,
            &IVec2::new(0, 2),
            &IVec2::new(4, 2),
            100,
        );

        // Should find a path going around through y=0
        assert!(!path.is_empty());
        assert_eq!(path.first(), Some(&IVec2::new(0, 2)));
        assert_eq!(path.last(), Some(&IVec2::new(4, 2)));
        // Path should avoid the blocked cells
        assert!(path.iter().all(|p| !(p.x == 2 && p.y >= 1)));
    }

    #[test]
    fn test_a_star_to_zero_steps() {
        let grid = Grid::new(IVec2::new(5, 5));
        let path = grid.a_star_to(&EntityKind::Unit, &IVec2::new(0, 0), &IVec2::new(4, 4), 0);

        // With 0 steps, should only include the starting position
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], IVec2::new(0, 0));
    }

    #[test]
    fn test_a_star_to_adjacent() {
        let grid = Grid::new(IVec2::new(5, 5));
        let path = grid.a_star_to(
            &EntityKind::Unit,
            &IVec2::new(2, 2),
            &IVec2::new(2, 3),
            100,
        );

        assert_eq!(path.len(), 2);
        assert_eq!(path[0], IVec2::new(2, 2));
        assert_eq!(path[1], IVec2::new(2, 3));
    }
}
