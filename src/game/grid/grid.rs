use std::collections::VecDeque;

use bevy::prelude::*;
use pathfinding::prelude::astar;

use super::selection;
use crate::util::cords;

// Grid is a 2D fixed-size grid that stores values of type T.
#[derive(Clone, Debug, Reflect)]
pub struct Grid<T: Default> {
    size: IVec2,
    data: Vec<T>,
}

impl<T: Default> Grid<T> {
    // Creates a new Grid with the specified size.
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

    pub fn iter_in_order(&self) -> impl Iterator<Item = IVec2> {
        let size = self.size;
        (0..self.data.len()).map(move |i| cords::index_to_location(&size, i))
    }

    // Breath-first iterator that explores the grid breath-first from a starting point starting in a given direction.
    pub fn iter_breath(
        &self,
        start: IVec2,
        direction: IVec2,
        selection: selection::Shape,
    ) -> impl Iterator<Item = IVec2> {
        let mut queue = VecDeque::with_capacity(self.data.len());
        queue.push_back(start.clone());
        let mut visited = vec![false; self.data.len()];
        let size = self.size;
        let dir = if direction == IVec2::ZERO {
            IVec2::new(1, 0)
        } else {
            direction
        };
        let rotated = IVec2::new(-direction.y, direction.x); // Rotate direction 90 degrees clockwise
        std::iter::from_fn(move || {
            while let Some(location) = queue.pop_front() {
                if location.x < 0 || location.y < 0 {
                    continue;
                }
                if location.x >= size.x || location.y >= size.y {
                    continue;
                }
                let index = cords::location_to_index(&size, &location);
                if !visited[index] && selection.contains(&location) {
                    visited[index] = true;
                    queue.extend(
                        [
                            location + dir,
                            location + rotated,
                            location - dir,
                            location - rotated,
                        ]
                        .into_iter(),
                    );
                    return Some(location);
                }
            }
            None
        })
    }

    // A* pathfinding algorithm to find a path from start to end.
    pub fn a_star(&self, start: &IVec2, end: &IVec2, valid: impl Fn(&T) -> bool) -> Vec<IVec2> {
        if let Some((path, _)) = astar(
            start,
            |p| {
                vec![
                    p + IVec2::new(1, 0),
                    p + IVec2::new(-1, 0),
                    p + IVec2::new(0, 1),
                    p + IVec2::new(0, -1),
                ]
                .into_iter()
                .filter(|location| {
                    if location == end {
                        return true;
                    }
                    if let Some(cell) = self.get(location) {
                        return valid(cell);
                    }
                    false
                })
                .map(move |location| (location, 1))
            },
            |p| p.distance_squared(*end),
            |p| p == end,
        ) {
            path
        } else {
            Vec::new()
        }
    }

    fn index(&self, location: &IVec2) -> Option<usize> {
        if cords::location_within(&IVec2::ZERO, &self.size, location) {
            Some(cords::location_to_index(&self.size, location))
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
    }

    #[test]
    fn test_get_set_take() {
        let mut grid = Grid::<i32>::new(IVec2::new(3, 3));
        assert_eq!(grid.get(&IVec2::new(0, 0)), Some(&0));
        assert_eq!(grid.set(&IVec2::new(0, 0), 1), Some(&1));
        assert_eq!(grid.get(&IVec2::new(0, 0)), Some(&1));
        assert_eq!(grid.take(&IVec2::new(0, 0)), Some(1));
        assert_eq!(grid.get(&IVec2::new(0, 0)), Some(&0));
        assert_eq!(grid.get(&IVec2::new(-1, 0)), None);
    }

    #[test]
    fn test_a_star() {
        let grid = Grid::<Option<()>>::new(IVec2::new(5, 5));
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
    fn test_iter() {
        let grid = Grid::<()>::new(IVec2::new(3, 3));
        let mut iter = grid.iter_breath(IVec2::new(1, 1), IVec2::new(1, 0), selection::Shape::All);
        assert_eq!(iter.next(), Some(IVec2::new(1, 1)));
        assert_eq!(iter.next(), Some(IVec2::new(2, 1)));
        assert_eq!(iter.next(), Some(IVec2::new(1, 2)));
        assert_eq!(iter.next(), Some(IVec2::new(0, 1)));
        assert_eq!(iter.next(), Some(IVec2::new(1, 0)));
        assert_eq!(iter.next(), Some(IVec2::new(2, 2)));
        assert_eq!(iter.next(), Some(IVec2::new(2, 0)));
        assert_eq!(iter.next(), Some(IVec2::new(0, 2)));
        assert_eq!(iter.next(), Some(IVec2::new(0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_corner() {
        let grid = Grid::<()>::new(IVec2::new(3, 3));
        let mut iter = grid.iter_breath(IVec2::new(0, 0), IVec2::new(1, 0), selection::Shape::All);
        assert_eq!(iter.next(), Some(IVec2::new(0, 0)));
        assert_eq!(iter.next(), Some(IVec2::new(1, 0)));
        assert_eq!(iter.next(), Some(IVec2::new(0, 1)));
        assert_eq!(iter.next(), Some(IVec2::new(2, 0)));
        assert_eq!(iter.next(), Some(IVec2::new(1, 1)));
        assert_eq!(iter.next(), Some(IVec2::new(0, 2)));
        assert_eq!(iter.next(), Some(IVec2::new(2, 1)));
        assert_eq!(iter.next(), Some(IVec2::new(1, 2)));
        assert_eq!(iter.next(), Some(IVec2::new(2, 2)));
        assert_eq!(iter.next(), None);
    }
}
