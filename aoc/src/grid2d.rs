use smallvec::SmallVec;

#[derive(Debug)]
pub struct Grid2D<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: From<char>> From<&str> for Grid2D<T> {
    fn from(input: &str) -> Self {
        let data: Vec<Vec<T>> = input
            .lines()
            .map(|line| line.chars().map(T::from).collect())
            .collect();
        let width = data[0].len();
        let height = data.len();

        Self {
            data,
            width,
            height,
        }
    }
}

impl<T> Grid2D<T> {
    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        0 <= x && x < self.width as isize && 0 <= y && y < self.height as isize
    }

    pub fn all_neighbors(&self, x: usize, y: usize) -> SmallVec<[((usize, usize), T); 8]>
    where
        T: Clone,
    {
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
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if self.in_bounds(nx, ny) {
                    Some((
                        (nx as usize, ny as usize),
                        self.data[ny as usize][nx as usize].clone(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn cardinal_neighbors(&self, x: usize, y: usize) -> SmallVec<[((usize, usize), T); 4]>
    where
        T: Clone,
    {
        const CARDINALS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        CARDINALS
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if self.in_bounds(nx, ny) {
                    Some((
                        (nx as usize, ny as usize),
                        self.data[ny as usize][nx as usize].clone(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[y][x]
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
        if self.y >= self.grid.height {
            return None;
        }

        let item = &self.grid.data[self.y][self.x];
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
