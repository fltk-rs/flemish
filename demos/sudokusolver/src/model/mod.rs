pub struct Model {
    pub grid: [[i32; 9]; 9],
}

impl Model {
    pub fn default() -> Self {
        Self { grid: [[0; 9]; 9] }
    }
    pub fn clear(&mut self) {
        self.grid = [[0; 9]; 9];
    }
    fn solvable(&mut self) -> bool {
        let mut items: [i32; 9];

        for row in self.grid {
            items = [0; 9];
            for value in row {
                if value > 0 && value < 10 {
                    items[(value - 1) as usize] += 1;
                }
            }
            if items.iter().any(|&n| n > 1) {
                return false;
            }
        }

        for i in 0..9 {
            items = [0; 9];
            for row in self.grid {
                if row[i] > 0 && row[i] < 10 {
                    items[(row[i] - 1) as usize] += 1;
                }
            }
            if items.iter().any(|&n| n > 1) {
                return false;
            }
        }

        for &x in [0, 3, 6].iter() {
            for &y in [0, 3, 6].iter() {
                items = [0; 9];
                for i in 0..3 {
                    for j in 0..3 {
                        if self.grid[y + i][x + j] > 0 && self.grid[y + i][x + j] < 10 {
                            items[(self.grid[y + i][x + j] - 1) as usize] += 1;
                        }
                    }
                }
                if items.iter().any(|&n| n > 1) {
                    return false;
                }
            }
        }
        true
    }
    fn possible(&self, y: usize, x: usize, number: i32) -> bool {
        if self.grid[y].iter().any(|&n| n == number) {
            return false;
        }

        if self.grid.iter().any(|n| n[x] == number) {
            return false;
        }

        let x0: usize = (x / 3) * 3;
        let y0: usize = (y / 3) * 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.grid[y0 + i][x0 + j] == number {
                    return false;
                }
            }
        }
        true
    }
    fn find_next_cell2fill(&self) -> (usize, usize) {
        for (x, row) in self.grid.iter().enumerate() {
            for (y, &val) in row.iter().enumerate() {
                if val == 0 {
                    return (x, y);
                }
            }
        }
        (99, 99)
    }
    fn solve(&mut self) -> bool {
        let (i, j) = self.find_next_cell2fill();
        if i == 99 {
            return true;
        }
        for e in 1..10 {
            if self.possible(i, j, e) {
                self.grid[i][j] = e;
                if self.solve() {
                    return true;
                }
                self.grid[i][j] = 0;
            }
        }
        false
    }
    pub fn answer(&mut self) {
        if self.solvable() {
            self.solve();
        } else {
            self.clear();
        }
    }
}
