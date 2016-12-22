use std::fmt;

#[derive(Clone)]
pub struct Grid {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
    dx: f64,
    dy: f64,
    cells_x: u32,
    cells_y: u32,
}

#[derive(Clone)]
pub struct GridStrip {
    pub start: u32,
    pub height: u32,
}

pub struct GridIter<'a> {
    grid: &'a Grid,
    x: f64,
    y: f64,
}

pub struct StripIter<'a> {
    grid: &'a Grid,
    strip_height: u32, 
    cur_pos: u32,
}

pub struct GridStripIter<'a> {
    grid: &'a Grid,
    grid_strip: GridStrip,
    grid_iter: GridIter<'a>
}

impl Grid {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64,
               cells_x: u32, cells_y: u32) -> Self {
        let width = right-left;
        let height = top-bottom;
        Grid{left: left, top: top, right: right, bottom: bottom,
            dx: width / (cells_x as f64), dy: -height / (cells_y as f64),
            cells_x: cells_x, cells_y: cells_y}
    }

    #[inline]
    pub fn num_cells(&self) -> usize {
        (self.cells_x as usize) * (self.cells_y as usize)
    }

    #[inline]
    pub fn num_cells_in_strip(&self, strip: &GridStrip) -> usize {
        (strip.height as usize) * (self.cells_x as usize)
    }

    #[inline]
    pub fn cells_wide(&self) -> u32 {
        self.cells_x
    }
    
    #[inline]
    pub fn cells_high(&self) -> u32 {
        self.cells_y
    }

    #[inline]
    pub fn row_start(&self, row: u32) -> usize {
        if row >= self.cells_y {
            panic!("Row index out of bound");
        }
        (self.cells_y * row) as usize
    }

    #[inline]
    pub fn first_cell_position(&self) -> (f64, f64) {
        (self.left + self.dx * 0.5, self.top + self.dy * 0.5)
    }

    pub fn cell_position(&self, x: u32, y: u32) -> Option<(f64, f64)> {
        if x < self.cells_x && y < self.cells_y {
            let x = self.left + ((x as f64) + 0.5) * self.dx;
            let y = self.top + ((y as f64) + 0.5) * self.dy;
            Some((x, y))
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self) -> GridIter<'a> {
        GridIter::new(self)
    }
}

impl<'a> Grid {
    pub fn iter_strips(&'a self, height: u32) -> StripIter<'a> {
        StripIter::new(self, 0, height)
    }
}

impl<'a> GridIter<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        let first_cell = grid.first_cell_position();
        GridIter{grid: grid, x: first_cell.0, y: first_cell.1}
    }
    pub fn from_row(grid: &'a Grid, row: u32) -> Self {
        let (x, y) = grid.cell_position(0, row).unwrap();
        GridIter{grid: grid, x: x, y: y}
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

impl fmt::Display for GridStrip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Strip<Rows {} to {}>", self.start, self.start+self.height-1)
    }
}

impl GridStrip {
    pub fn new(start: u32, height: u32) -> Self {
        GridStrip{start: start, height: height}
    }

}

impl<'a> GridStrip {
    pub fn iter(&self, grid: &'a Grid) -> GridStripIter<'a> {
        GridStripIter::new(grid, self.clone())
    }
}

impl<'a> GridStripIter<'a> {
    pub fn new(grid: &'a Grid, strip: GridStrip) -> Self {
        let grid_iter = GridIter::from_row(grid, strip.start);

        GridStripIter{grid: grid, grid_strip: strip, grid_iter: grid_iter}
    }
}

impl<'a> Iterator for GridStripIter<'a> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.grid_iter.next();
        
        match next {
            Some((x,y)) => {
                let end_row = self.grid_strip.start + self.grid_strip.height-1;
                let max_y = self.grid.top + self.grid.dy*(0.5+(end_row as f64));
                if y < max_y  {
                    None
                } else {
                    Some((x,y))
                }
            },
            None => None,
        }
    }
}

impl<'a> StripIter<'a> {
    pub fn new(grid: &'a Grid, start_pos: u32, strip_height: u32) -> Self {
        StripIter{grid: grid, strip_height: strip_height, cur_pos: start_pos}
    }
}

impl<'a> Iterator for StripIter<'a> {
    type Item = GridStrip;

    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = 
            if self.cur_pos + self.strip_height >= self.grid.cells_high() {
                if self.cur_pos+1 == self.grid.cells_high() {
                    self.grid.cells_high()
                } else {
                    self.grid.cells_high()-1
                }
            } else {
                self.cur_pos + self.strip_height
            };
        
        if self.cur_pos >= self.grid.cells_high() {
            None
        } else {
            let pos = self.cur_pos;
            let height = next_pos - self.cur_pos;
            self.cur_pos = next_pos;
            Some(GridStrip::new(pos, height))
        }
    }
}
