use rand::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Cell {
    state: i8,
}
#[derive(Debug, Clone)]
pub struct Grid {
    pub xmax: usize,
    pub ymax: usize,
    xmin: usize,
    ymin: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(x: usize, y: usize, probability: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut outer: Vec<Vec<Cell>> = Vec::with_capacity(y);
        for _ in 0..y {
            let mut inner: Vec<Cell> = Vec::with_capacity(x);
            for _ in 0..x {
                let rng_value: f64 = rng.gen();
                let state: i8 = if rng_value > probability { 1 } else { 0 };
                let cell = Cell::new(state);
                inner.push(cell);
            }
            outer.push(inner);
        }
        Grid {
            xmax: x,
            ymax: y,
            xmin: 0,
            ymin: 0,
            cells: outer,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }
    pub fn set_cell(&mut self, cell: Cell, x: usize, y: usize) {
        self.cells[y][x] = cell
    }
    pub fn get_cells(&mut self) -> &Vec<Vec<Cell>> {
        &self.cells
    }

    fn get_states(&mut self) -> Vec<Vec<i8>> {
        self.get_cells()
            .iter()
            .map(|row| row.iter().map(|cell| cell.state).collect())
            .collect()
    }

    pub fn get_neighbors(&mut self, x: usize, y: usize) -> i8 {
        let xmax_bound = self.xmax - 1;
        let ymax_bound = self.ymax - 1;

        // if try to go over an edge, wrap around
        let up_y = if y == ymax_bound { self.ymin } else { y + 1 };
        let up = self.get_cell(x, up_y).state;

        let down_y = if y == self.ymin { ymax_bound } else { y - 1 };
        let down = self.get_cell(x, down_y).state;

        let right_x = if x == xmax_bound { self.xmin } else { x + 1 };
        let right = self.get_cell(right_x, y).state;

        let left_x = if x == self.xmin { xmax_bound } else { x - 1 };
        let left = self.get_cell(left_x, y).state;

        up + down + left + right
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = String::from("");
        for row in self.clone().get_states() {
            for state in row {
                message = message + format!("{}", state).as_str();
            }
            message = message + "\n";
        }
        write!(f, "{}", message)
    }
}

impl Cell {
    fn new(state: i8) -> Self {
        Self { state }
    }

    pub fn update(&self, neighbors: i8) -> Self {
        let new_state = match self.state {
            1 if (neighbors < 2 || neighbors > 3) => 0,
            0 if (neighbors == 3) => 1,
            _ => self.state,
        };
        Self { state: new_state }
    }
}
