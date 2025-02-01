use crate::screen::*;

#[derive(Debug, Clone)]
pub struct SmokeGrid {
    width: usize,
    height: usize,
    decay_rate: f64,
    grid: Vec<Vec<f64>>,
}

impl SmokeGrid {
    pub fn new(width: usize, height: usize, decay_rate: f64) -> Self {
        let mut grid = vec![vec![0.; width]; height];

        // Boundary condition:
        // - grid[0][x] = 1.0
        // - and 0.0 on the other boundaries
        for x in 0..width - 1 {
            grid[0][x] = 1.0;
        }

        SmokeGrid { width, height, grid, decay_rate }
    }

    pub fn diffuse(&mut self, dt: f64, diff_rate: f64) {
        let mut new_grid = self.grid.clone();
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let diff_x = self.grid[y][x + 1] + self.grid[y][x - 1] - 2.0 * self.grid[y][x];
                let diff_y = self.grid[y + 1][x] + self.grid[y - 1][x] - 2.0 * self.grid[y][x];

                new_grid[y][x] += diff_rate * dt * (diff_x + diff_y);
            }
        }

        // Simulate dissipation
        for x in 0..self.width - 1 {
            //println!("grid[{}][{}] - {:.2} = {:.2} - {:.2} = {:.2}", 0, x, self.decay_rate, self.grid[0][x], self.decay_rate, self.grid[0][x] - self.decay_rate);
            new_grid[0][x] = (self.grid[0][x] - self.decay_rate).max(0.0);
            //println!("grid[{}][{}] = {:.2}", 0, x, self.grid[0][x]);
        }

        self.grid = new_grid;
    }

    pub fn display_debug(&self) {
        // Top border
        print!("{}┌", SHIFT_MINUS_ONE);
        for _ in 0..self.width {
            print!("─");
        }
        print!("┐\n");
        for y in (0..self.height).rev() {
            print!("{}│", SHIFT_MINUS_ONE);
            for x in 0..self.width {
                let cell = self.grid[y][x];
                let char_rep = if cell > 0.8 {
                    //'@'
                    '█'
                } else if cell > 0.6 {
                    //'*'
                    '▓'
                } else if cell > 0.4 {
                    //'+'
                    '▒'
                } else if cell > 0.2 {
                    //'.'
                    '░'
                } else {
                    ' '
                };
                print!("{}", char_rep);
            }
            print!("│\n");
        }
    }

    pub fn display(&self) {
        for y in (0..self.height).rev() {
            print!("{}", SHIFT);
            for x in 0..self.width {
                let cell = self.grid[y][x];
                let char_rep = if cell > 0.8 {
                    //'@'
                    '█'
                } else if cell > 0.6 {
                    //'*'
                    '▓'
                } else if cell > 0.4 {
                    //'+'
                    '▒'
                } else if cell > 0.2 {
                    //'.'
                    '░'
                } else {
                    ' '
                };
                print!("{}", char_rep);
            }
            println!();
        }
    }
}
