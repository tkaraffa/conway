use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self};
use ggez::{Context, GameResult};
use rand::prelude::*;
#[derive(Debug, Clone)]
struct Cell {
    state: i8,
    x: usize,
    y: usize,
}
#[derive(Debug, Clone)]
struct Grid {
    xmax: usize,
    ymax: usize,
    xmin: usize,
    ymin: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(x: usize, y: usize, probability: f64) -> Self {
        let mut rng = rand::thread_rng();
        let size = (x as i32) * (y as i32);
        let mut cells: Vec<Cell> = Vec::with_capacity(size as usize);
        for j in 0..y {
            for i in 0..x {
                let rng_value: f64 = rng.gen();
                let state: i8 = if rng_value > probability { 1 } else { 0 };
                let cell = Cell::new(state, i, j);
                cells.push(cell);
            }
        }
        Grid {
            xmax: x,
            ymax: y,
            xmin: 0,
            ymin: 0,
            cells,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.xmax + x]
    }
    fn set_cell(&mut self, cell: &Cell) {
        self.cells[cell.y * self.xmax + cell.x] = cell.clone()
    }
    fn get_cells(&mut self) -> &Vec<Cell> {
        &self.cells
    }

    fn get_neighbors(&mut self, x: usize, y: usize) -> i8 {
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

impl Cell {
    fn new(state: i8, x: usize, y: usize) -> Self {
        Self { state, x, y }
    }

    pub fn update(&self, neighbors: i8) -> Self {
        let new_state = match self.state {
            1 if (neighbors < 2 || neighbors > 3) => 0,
            0 if (neighbors == 3) => 1,
            _ => self.state,
        };
        Self {
            state: new_state,
            x: self.x,
            y: self.y,
        }
    }
}

pub struct MainState {
    grid: Grid,
}

impl MainState {
    pub fn new(xmax: usize, ymax: usize, probability: f64) -> GameResult<MainState> {
        let s = MainState {
            grid: Grid::new(xmax, ymax, probability),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut current_grid = self.grid.clone();
        for y in 0..self.grid.ymax {
            for x in 0..self.grid.xmax {
                let cell = {
                    let neighbors = current_grid.get_neighbors(x, y).clone();
                    let new_cell = current_grid.get_cell(x, y);
                    new_cell.update(neighbors)
                };
                self.grid.set_cell(&cell);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
        let dead_color = [0.0, 0.0, 0.0, 1.0];
        let alive_color = [1.0, 1.0, 1.0, 1.0];

        let scale = 4.0;

        self.grid.get_cells().iter().for_each(|cell| {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(graphics::Rect::new(
                        cell.x as f32 * scale,
                        cell.y as f32 * scale,
                        scale,
                        scale,
                    ))
                    .color(if cell.state == 1 {
                        alive_color
                    } else {
                        dead_color
                    }),
            );
        });

        canvas.finish(ctx)?;
        Ok(())
    }
}
