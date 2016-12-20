
pub struct Grid {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
    width: f64,
    height: f64,
    dx: f64,
    dy: f64,
    cells_x: u32,
    cells_y: u32,
}

pub struct GridIter<'a> {
    grid: &'a Grid,
    x: f64,
    y: f64,
}

impl Grid {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64,
               cells_x: u32, cells_y: u32) -> Self {
        let width = right-left;
        let height = top-bottom;
        Grid{left: left, top: top, right: right, bottom: bottom,
            width: width, height: height, 
            dx: width / (cells_x as f64), dy: -height / (cells_y as f64),
            cells_x: cells_x, cells_y: cells_y}
    }

    #[inline]
    pub fn num_cells(&self) -> usize {
        (self.cells_x as usize) * (self.cells_y as usize)
    }

    #[inline]
    pub fn cells_wide(&self) -> u32 {
        self.cells_x
    }
    
    #[inline]
    pub fn cells_high(&self) -> u32 {
        self.cells_y
    }

    pub fn cell_position(&self, x: u32, y: u32) -> Option<(f64, f64)> {
        if x < self.cells_x && y < self.cells_y {
            let x = ((x as f64) + 0.5) * self.dx;
            let y = ((y as f64) + 0.5) * self.dy;
            Some((x, y))
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self) -> GridIter<'a> {
        GridIter::new(self)
    }
}

impl<'a> GridIter<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        GridIter{grid: grid, x: grid.left + grid.dx * 0.5, y: grid.top + grid.dy * 0.5}
    }
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x;
        let y = self.y;

        self.x += self.grid.dx;
        if self.x >= self.grid.right {
            self.x = self.grid.left + self.grid.dx * 0.5;
            self.y += self.grid.dy;
        }
        if y <= self.grid.bottom {
            None
        } else {
            Some((x, y))
        }
    }
}
