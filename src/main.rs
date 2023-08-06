mod conway;
use conway::Grid;

fn main() {
    let xmax = 100;
    let ymax = 100;

    let probability = 0.5;
    let cycles = 50;
    let mut grid = Grid::new(xmax, ymax, probability);

    for i in 0..cycles {
        let mut current_grid = grid.clone();
        println!("{}", current_grid);
        for j in 0..ymax {
            for i in 0..xmax {
                let cell = {
                    let neighbors = current_grid.get_neighbors(i, j).clone();
                    let new_cell = current_grid.get_cell(i, j);
                    new_cell.update(neighbors)
                };
                grid.set_cell(cell, i, j);
            }
        }
    }
}
