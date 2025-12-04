use smallvec::SmallVec;

#[derive(Debug)]
pub struct Grid2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: From<char>> From<&str> for Grid2D<T> {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();

        let first = lines.next().unwrap_or("");
        let width = first.len();

        let mut data = Vec::new();
        data.reserve(width);

        for ch in first.chars() {
            data.push(T::from(ch));
        }

        let mut height = 1;

        for line in lines {
            assert!(line.len() == width, "Line width mismatch");
            height += 1;

            for ch in line.chars() {
                data.push(T::from(ch));
            }
        }

        Self {
            data,
            width,
            height,
        }
    }
}

impl<T> Grid2D<T> {
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[self.index(x, y)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn all_neighbors(&self, x: usize, y: usize) -> SmallVec<[((usize, usize), &T); 8]> {
        const NEIGHBORS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (0, -1),
        ];

        NEIGHBORS
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                Some(((nx, ny), self.get(nx, ny)?))
            })
            .collect()
    }

    pub fn cardinal_neighbors(&self, x: usize, y: usize) -> SmallVec<[((usize, usize), &T); 4]> {
        const CARDINALS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        CARDINALS
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                Some(((nx, ny), self.get(nx, ny)?))
            })
            .collect()
    }

    pub fn iter(&self) -> Grid2DIter<'_, T> {
        Grid2DIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct Grid2DIter<'a, T> {
    grid: &'a Grid2D<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for Grid2DIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.y * self.grid.width + self.x;
        if idx >= self.grid.data.len() {
            return None;
        }

        let item = &self.grid.data[idx];
        let coords = (self.x, self.y);

        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        Some((coords, item))
    }
}

impl<'a, T> IntoIterator for &'a Grid2D<T> {
    type Item = ((usize, usize), &'a T);
    type IntoIter = Grid2DIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
