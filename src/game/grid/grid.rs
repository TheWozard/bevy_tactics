use bevy::prelude::*;
use pathfinding::prelude::astar;
use std::collections::VecDeque;

use crate::util::cords;
use super::selection;

// Grid is a 2D fixed-size grid that stores values of type T.
#[derive(Clone, Debug, Reflect)]
pub struct Grid<T: Default> {
    size: IVec2,
    data: Vec<T>,
}

impl<T: Default> Grid<T> {
    // Creates a new Grid with the specified size. Negative sizes are treated as positive.
    pub fn new(size: IVec2) -> Self {
        let mut data = Vec::<T>::new();
        data.resize_with((size.x * size.y) as usize, || T::default());
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
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, location: &IVec2) -> Option<&mut T> {
        if let Some(index) = self.index(location) {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    // Sets the value at the specified location, returning a reference to the value if successful.
    pub fn set(&mut self, location: &IVec2, value: T) -> Option<&T> {
        if let Some(index) = self.index(location) {
            self.data[index] = value;
            Some(&self.data[index])
        } else {
            None
        }
    }

    // Takes the value at the specified location, returning it as owned.
    pub fn take(&mut self, location: &IVec2) -> Option<T> {
        if let Some(index) = self.index(location) {
            Some(std::mem::replace(&mut self.data[index], T::default()))
        } else {
            None
        }
    }

    // Breath-first iterator that explores the grid breath-first from a starting point starting in a given direction.
    pub fn iter(
        &self,
        start: &IVec2,
        direction: &IVec2,
        selection: selection::Shape,
    ) -> impl Iterator<Item = usize> + '_ {
        let mut queue = VecDeque::with_capacity(self.data.len());
        queue.push_back(start.clone());
        let mut visited = vec![false; self.data.len()];
        let size = self.size;
        let len = self.data.len();
        let dir = direction.clone();
        let rotated = IVec2::new(-direction.y, direction.x); // Rotate direction 90 degrees clockwise
        std::iter::from_fn(move || {
            while let Some(location) = queue.pop_front() {
                let index = cords::location_to_index(&size, &location);
                if index >= len {
                    continue;
                }
                if !visited[index] && selection.contains(&location) {
                    visited[index] = true;
                    queue.extend(
                        [
                            location + dir,
                            location + rotated,
                            location - dir,
                            location - rotated,
                        ]
                        .into_iter()
                    );
                    return Some(index);
                }
            }
            None
        })
    }

    // A* pathfinding algorithm to find a path from start to end.
    pub fn a_star(
        &self,
        start: &IVec2,
        end: &IVec2,
        valid: impl Fn(Option<&T>) -> bool,
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
                    if self.within(&next) && (&next == end || valid(self.get(&next))) {
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

    fn index(&self, location: &IVec2) -> Option<usize> {
        if cords::location_within(&IVec2::ZERO, &self.size, location) {
            Some(cords::location_to_index(&self.size, location))
        } else {
            None
        }
    }

    fn location(&self, index: usize) -> Option<IVec2> {
        if index < self.data.len() {
            Some(cords::index_to_location(&self.size, index))
        } else {
            None
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
    fn test_iter_breath_first() {
        let grid = Grid::<()>::new(IVec2::new(3, 3));
        let mut iter =
            grid.iter(&IVec2::new(1, 1), &IVec2::new(1, 0), selection::Shape::All);
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
            grid.iter(&IVec2::new(0, 0), &IVec2::new(1, 0), selection::Shape::All);
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
