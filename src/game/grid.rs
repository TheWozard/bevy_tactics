use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use super::utils;

pub fn plugin(app: &mut App) {
    app.add_observer(on_remove_grid_location);

    app.register_type::<Grid>();
    app.register_type::<GridLocation>();
    app.register_type::<GridContent>();
    app.register_type::<GridOwner>();
    app.register_type::<GridOwned>();
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Transform, Name::new("Grid"))]
pub struct Grid {
    pub grid: utils::Grid<Entity>,
}

#[derive(Clone, Debug, Reflect)]
pub struct GridContent {
    pub tile: Entity,
}

impl Grid {
    pub fn new(size: IVec2) -> Self {
        Grid {
            grid: utils::Grid::new(size),
        }
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        location: &IVec2,
        bundle: impl Bundle,
    ) -> Option<Entity> {
        if self.grid.get(location).is_none() {
            self.grid
                .set(
                    location,
                    commands
                        .spawn((GridLocation::new(location.clone()), bundle))
                        .id(),
                )
                .copied()
        } else {
            None
        }
    }

    pub fn move_to(&mut self, from: &IVec2, to: &IVec2) -> Option<GridLocation> {
        if self.grid.within(to) && self.grid.get(from).is_some() {
            if self.grid.get(to).is_none() {
                let entity = self.grid.take(from).unwrap();
                self.grid.set(to, entity);
                return Some(GridLocation::new(to.clone()));
            }
        }
        None
    }

    pub fn get(&self, location: &IVec2) -> Option<Entity> {
        self.grid.get(location).copied()
    }

    fn clear(&mut self, location: &IVec2) {
        self.grid.take(location);
    }

    pub fn find(
        &self,
        location: &IVec2,
        direction: &IVec2,
        predicate: impl Fn(Entity) -> bool,
    ) -> Option<IVec2> {
        let mut iter = self.grid.iter_breath_first(location, direction).skip(1);
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

fn on_remove_grid_location(
    trigger: Trigger<OnRemove, GridLocation>,
    location_query: Query<(&GridLocation, &GridOwner)>,
    mut grid_query: Query<&mut Grid>,
) {
    if let Ok((grid_location, grid_owned)) = location_query.get(trigger.target()) {
        if let Ok(mut grid) = grid_query.get_mut(grid_owned.get()) {
            grid.clear(grid_location.as_ivec2());
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

    pub fn as_ivec2(&self) -> &IVec2 {
        &self.location
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
