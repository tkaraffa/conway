fn main() {
    println!("Hello, world!");
    let g = Grid::new(3, 3);
    let i = g.get_point(1, 4);
    println!("{:?}", i);
}
/*
things needed:
grid of X by Y
something to indicate if it's alive or dead
rules for how to update
each point needs to be aware of its neighbors

*/
#[derive(Debug)]
struct Point {
    x: usize, 
    y: usize,
}
#[derive(Debug)]
struct Cell {
    state: i8,
    position: Point
}

struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn new(x: usize, y: usize) -> Self {
        let mut outer: Vec<Vec<Cell>> = Vec::with_capacity(y);
        for y in 0..y {
            let mut inner: Vec<Cell> = Vec::with_capacity(x);
            for x in 0..x {
                let cell = Cell::new(x, y);
                inner.push(cell);
            }
            outer.push(inner);
        }
        Grid(outer)
    }
    fn get_point(&self, x: usize, y: usize) -> &Cell {
        &self.0[y][x]
    }

}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Cell {
            state: 0,
            position: Point {x, y}

        }
    }


    fn neighbors(&self) -> usize {
        // self.up.state + self.down.state + self.right.state + self.left.state
        0
    }

    fn update(&mut self) {
        let neighbors = self.neighbors();
        if self.state == 1 {
            if neighbors < 2 || neighbors > 3 {
                self.state = 0;
            }
        } else if self.state == 0 {
            if neighbors == 3 {
                self.state = 1;
            }
        }
    }
}