use bevy::prelude::*;

pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Startup, Grid::create::<16>);
    app.add_systems(FixedUpdate, Grid::gizmo);
}

#[derive(Component)]
pub struct Location {
    cell: UVec2,
}

impl Default for Location {
    fn default() -> Self {
        Location { cell: UVec2::ZERO }
    }
}

impl Location {
    pub fn cell(&self) -> UVec2 {
        self.cell
    }
}

#[derive(Component)]
#[require(Transform, Scale)]
pub struct Grid {
    size: UVec2,
    data: Vec<Option<Entity>>,
}

impl Grid {
    pub fn create<const SIZE: u32>(mut commands: Commands) {
        commands.spawn((Grid::new(UVec2::new(SIZE, SIZE)),));
    }

    pub fn gizmo(mut gizmos: Gizmos, query: Query<(&Grid, &Scale, &Transform)>) {
        for (grid, scale, transform) in query.iter() {
            gizmos.grid_2d(
                transform.translation.truncate(),
                grid.size,
                scale.scale,
                Color::srgb(0.0, 1.0, 0.0),
            );
        }
    }

    pub fn new(size: UVec2) -> Self {
        let data = vec![None; (size.x * size.y) as usize];
        Grid { size, data }
    }

    pub fn get_size(&self) -> UVec2 {
        self.size
    }

    // Returns true if the cell is empty and valid.
    pub fn is_empty_cell(&self, cell: UVec2) -> bool {
        if self.is_valid_cell(cell) {
            self.data[self.index(cell)].is_none()
        } else {
            false
        }
    }

    // Returns the entity at the given coordinates. Returns None if the cell is invalid or empty.
    pub fn get_cell(&self, cell: UVec2) -> Option<Entity> {
        if self.is_valid_cell(cell) {
            self.data[self.index(cell)]
        } else {
            None
        }
    }

    // Sets the cell to the entity and updates the passed location. Clears the previous cell.
    // Returns true if the cell was set, false if it was invalid or already occupied.
    pub fn set_cell(&mut self, cell: UVec2, entity: &Entity, location: &mut Location) -> bool {
        if self.is_valid_cell(cell) {
            let index = self.index(cell);
            if self.data[index].is_none() {
                self.data[index] = Some(entity.clone());
                self.clear_cell(location.cell);
                location.cell = cell;
                return true;
            }
        }
        false
    }

    // Clears the cell at the given coordinates. Returns true if the cell was cleared, false if it was invalid or already empty.
    pub fn clear_cell(&mut self, cell: UVec2) -> bool {
        if self.is_valid_cell(cell) {
            let index = self.index(cell);
            if self.data[index].is_some() {
                self.data[index] = None;
                return true;
            }
        }
        false
    }

    fn is_valid_cell(&self, cell: UVec2) -> bool {
        cell.x < self.size.x && cell.y < self.size.y
    }

    fn index(&self, cell: UVec2) -> usize {
        (cell.x + cell.y * self.size.x) as usize
    }
}

#[derive(Component)]
pub struct Scale {
    pub scale: Vec2,
}

impl Default for Scale {
    fn default() -> Self {
        Scale {
            scale: Vec2::new(20.0, 20.0),
        }
    }
}
