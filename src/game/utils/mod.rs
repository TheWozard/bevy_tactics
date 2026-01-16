use std::collections::VecDeque;

use bevy::prelude::*;
use pathfinding::prelude::astar;

// SpaceGrid is a 2D Space implementation that uses a grid of fixed size to store values.
#[derive(Clone, Debug, Reflect)]
pub struct Grid<T> {
    size: IVec2,
    data: Vec<Option<T>>,
}

impl<T> Grid<T> {
    // Creates a new Grid with the specified size. Negative sizes are treated as positive.
    pub fn new(mut size: IVec2) -> Self {
        size = size.abs();
        let mut data = Vec::new();
        data.resize_with((size.x * size.y) as usize, || None);
        Grid { size, data }
    }

    // Returns true if the location is within the grid bounds.
    pub fn within(&self, location: &IVec2) -> bool {
        self.index(location).is_some()
    }

    // Returns the size of the grid.
    pub fn size(&self) -> IVec2 {
        self.size
    }

    // Returns the value at the specified location, if it exists.
    pub fn get(&self, location: &IVec2) -> Option<&T> {
        if let Some(index) = self.index(location) {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    // Sets the value at the specified location, returning a reference to the value if successful.
    pub fn set(&mut self, location: &IVec2, value: T) -> Option<&T> {
        if let Some(index) = self.index(location) {
            self.data[index] = Some(value);
            self.data[index].as_ref()
        } else {
            None
        }
    }

    // Takes the value at the specified location, returning it as owned.
    pub fn take(&mut self, location: &IVec2) -> Option<T> {
        if let Some(index) = self.index(location) {
            self.data[index].take()
        } else {
            None
        }
    }

    // Uses the passed iterator to find the location of the next available or used item.
    pub fn find(
        &self,
        iter: &mut impl Iterator<Item = usize>,
        predicate: impl Fn(Option<&T>) -> bool,
    ) -> Option<IVec2> {
        if let Some(index) = iter.find(|i| predicate(self.data[*i].as_ref())) {
            Some(self.location(index))
        } else {
            None
        }
    }

    pub fn a_star(
        &self,
        start: &IVec2,
        end: &IVec2,
        predicate: impl Fn(Option<&T>) -> bool,
    ) -> Vec<IVec2> {
        if let Some((path, _)) = astar(
            start,
            |p| {
                let mut successors = Vec::new();
                for dir in [
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(0, 1),
                    IVec2::new(0, -1),
                ] {
                    let next = p + dir;
                    if self.within(&next) && (&next == end || predicate(self.get(&next))) {
                        successors.push((next, 1));
                    }
                }
                successors
            },
            |p| p.distance_squared(*end),
            |p| p == end,
        ) {
            path.into_iter().collect()
        } else {
            Vec::new() // Return an empty path if no path is found
        }
    }

    // Converts an iterator of indices to an iterator of Vec3, applying the specified scale.
    pub fn index_to_vec2(&self, index: usize, scale: f32) -> Vec2 {
        let loc = self.location(index);
        self.location_to_vec2(&loc, scale)
    }

    pub fn location_to_vec2(&self, location: &IVec2, scale: f32) -> Vec2 {
        let scale = Vec2::splat(scale);
        let scale_offset = ((self.size.as_vec2() * scale) - scale) / 2.0;
        (location.as_vec2() * scale) - scale_offset
    }

    // --- Iterators ---

    pub fn iter_entire_grid(&self) -> impl Iterator<Item = usize> + use<T> {
        0..self.data.len()
    }

    // Breath-first iterator that explores the grid breath-first from a starting point starting in a given direction.
    pub fn iter_breath_first(
        &self,
        start: &IVec2,
        direction: &IVec2,
        selection: SelectionShape,
    ) -> impl Iterator<Item = usize> + use<T> {
        let mut queue = VecDeque::with_capacity(self.data.len());
        queue.push_back(start.clone());
        let mut visited = vec![false; (self.size.x * self.size.y) as usize];
        let size = self.size;
        let len = self.data.len();
        let mut dir = direction.clone();
        if dir == IVec2::ZERO || dir.length_squared() != 1 {
            dir = IVec2::new(1, 0); // Default direction if none is provided
        }
        let rotated = IVec2::new(-dir.y, dir.x); // Rotate direction 90 degrees clockwise
        std::iter::from_fn(move || {
            while let Some(current) = queue.pop_front() {
                let index = vec_to_index(&size, &current);
                if index >= len {
                    continue;
                }
                if !visited[index] && selection.contains(&current) {
                    visited[index] = true;
                    queue.extend(
                        [
                            current + dir,
                            current + rotated,
                            current - dir,
                            current - rotated,
                        ]
                        .into_iter()
                        .filter(|&loc| {
                            vec_within(&size, &loc) && !visited[vec_to_index(&size, &loc)]
                        }),
                    );
                    return Some(index);
                }
            }
            None
        })
    }

    // Converts an iterator of indices to an iterator of Transforms, applying the specified scale.
    pub fn iter_to_transform(
        &self,
        iter: impl Iterator<Item = usize>,
        scale: f32,
    ) -> impl Iterator<Item = Vec3> {
        let scale = Vec2::splat(scale);
        let scale_offset = ((self.size.as_vec2() * scale) - scale) / 2.0;
        iter.map(move |index| {
            let loc = self.location(index);
            ((loc.as_vec2() * scale) - scale_offset).extend(0.0)
        })
    }

    // Returns the index within the internal data vec the location maps to.
    pub fn index(&self, location: &IVec2) -> Option<usize> {
        if vec_within(&self.size, location) {
            Some(vec_to_index(&self.size, location))
        } else {
            None
        }
    }

    // Returns the location in the grid for a given index.
    // The index is wrapped around the grid size to ensure it stays within bounds.
    pub fn location(&self, mut index: usize) -> IVec2 {
        index = index % self.data.len();
        IVec2::new(
            (index % self.size.x as usize) as i32,
            (index / self.size.x as usize) as i32,
        )
    }
}

fn vec_to_index(size: &IVec2, location: &IVec2) -> usize {
    (location.y * size.x + location.x) as usize
}

fn vec_within(size: &IVec2, location: &IVec2) -> bool {
    location.x >= 0 && location.x < size.x && location.y >= 0 && location.y < size.y
}

pub struct SquareSelection {
    offset: IVec2,
    size: IVec2,
}

impl SquareSelection {
    pub fn new(start: IVec2, end: IVec2) -> Self {
        SquareSelection {
            offset: start,
            size: end - start,
        }
    }

    pub fn random_open<T>(&self, grid: &Grid<T>) -> Option<usize> {
        let index = rand::random_range(..self.size.x as usize * self.size.y as usize);
        let loc = IVec2::new(
            (index as i32 % self.size.x) + self.offset.x,
            (index as i32 / self.size.x) + self.offset.y,
        );
        grid.iter_breath_first(&loc, &IVec2::new(1, 0), SelectionShape::All)
            .find(|index| grid.data[*index].is_none())
    }
}

pub enum SelectionShape {
    All,
    Circle(Vec2, f32),
}

impl SelectionShape {
    pub fn contains(&self, location: &IVec2) -> bool {
        match self {
            SelectionShape::All => true,
            SelectionShape::Circle(center, radius) => {
                center.distance_squared(location.as_vec2()) <= radius * radius
            }
        }
    }
}

#[cfg(test)]
mod test_grid {
    use super::*;

    #[test]
    fn test_within() {
        assert_eq!(
            Grid::<()>::new(IVec2::new(3, 3)).within(&IVec2::new(0, 0)),
            true
        );
        assert_eq!(
            Grid::<()>::new(IVec2::new(3, 3)).within(&IVec2::new(1, 1)),
            true
        );
        assert_eq!(
            Grid::<()>::new(IVec2::new(3, 3)).within(&IVec2::new(2, 2)),
            true
        );
        assert_eq!(
            Grid::<()>::new(IVec2::new(3, 3)).within(&IVec2::new(3, 3)),
            false
        );
        assert_eq!(
            Grid::<()>::new(IVec2::new(3, 3)).within(&IVec2::new(-1, -1)),
            false
        );
    }

    #[test]
    fn test_size() {
        assert_eq!(Grid::<()>::new(IVec2::new(3, 3)).size(), IVec2::new(3, 3));
        assert_eq!(Grid::<()>::new(IVec2::new(-1, -1)).size(), IVec2::new(1, 1));
        assert_eq!(Grid::<()>::new(IVec2::new(3, -1)).size(), IVec2::new(3, 1));
        assert_eq!(Grid::<()>::new(IVec2::new(0, 0)).size(), IVec2::new(0, 0));
    }

    #[test]
    fn test_get_set_take() {
        let mut grid = Grid::new(IVec2::new(3, 3));
        assert_eq!(grid.get(&IVec2::new(0, 0)), None);
        assert_eq!(grid.set(&IVec2::new(0, 0), 42), Some(&42));
        assert_eq!(grid.get(&IVec2::new(0, 0)), Some(&42));
        assert_eq!(grid.take(&IVec2::new(0, 0)), Some(42));
        assert_eq!(grid.get(&IVec2::new(0, 0)), None);
    }

    #[test]
    fn test_a_star() {
        let grid = Grid::<()>::new(IVec2::new(5, 5));
        let path = grid.a_star(&IVec2::new(0, 0), &IVec2::new(4, 4), |cell| cell.is_none());
        assert_eq!(path.len(), 9); // Should find a path of length 9
        assert_eq!(path[0], IVec2::new(0, 0));
        assert_eq!(path[8], IVec2::new(4, 4));
    }
}

#[cfg(test)]
mod test_iter {
    use super::*;

    #[test]
    fn test_iter_entire_grid() {
        let grid = Grid::<()>::new(IVec2::new(3, 3));
        let mut iter = grid.iter_entire_grid();
        assert_eq!(iter.next(), Some(0)); // (0, 0)
        assert_eq!(iter.next(), Some(1)); // (0, 1)
        assert_eq!(iter.next(), Some(2)); // (0, 2)
        assert_eq!(iter.next(), Some(3)); // (1, 0)
        assert_eq!(iter.next(), Some(4)); // (1, 1)
        assert_eq!(iter.next(), Some(5)); // (1, 2)
        assert_eq!(iter.next(), Some(6)); // (2, 0)
        assert_eq!(iter.next(), Some(7)); // (2, 1)
        assert_eq!(iter.next(), Some(8)); // (2, 2)
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_breath_first() {
        let grid = Grid::<()>::new(IVec2::new(3, 3));
        let mut iter =
            grid.iter_breath_first(&IVec2::new(1, 1), &IVec2::new(1, 0), SelectionShape::All);
        assert_eq!(iter.next(), Some(4)); // (1, 1)
        assert_eq!(iter.next(), Some(5)); // (2, 1)
        assert_eq!(iter.next(), Some(7)); // (1, 2)
        assert_eq!(iter.next(), Some(3)); // (0, 1)
        assert_eq!(iter.next(), Some(1)); // (1, 0)
        assert_eq!(iter.next(), Some(8)); // (2, 2)
        assert_eq!(iter.next(), Some(2)); // (2, 0)
        assert_eq!(iter.next(), Some(6)); // (0, 2)
        assert_eq!(iter.next(), Some(0)); // (0, 0)
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_breath_first_corner() {
        let grid = Grid::<()>::new(IVec2::new(3, 3));
        let mut iter =
            grid.iter_breath_first(&IVec2::new(0, 0), &IVec2::new(1, 0), SelectionShape::All);
        assert_eq!(iter.next(), Some(0)); // (0, 0)
        assert_eq!(iter.next(), Some(1)); // (1, 0)
        assert_eq!(iter.next(), Some(3)); // (0, 1)
        assert_eq!(iter.next(), Some(2)); // (2, 0)
        assert_eq!(iter.next(), Some(4)); // (1, 1)
        assert_eq!(iter.next(), Some(6)); // (0, 2)
        assert_eq!(iter.next(), Some(5)); // (2, 1)
        assert_eq!(iter.next(), Some(7)); // (1, 2)
        assert_eq!(iter.next(), Some(8)); // (2, 2)
        assert_eq!(iter.next(), None);
    }
}
