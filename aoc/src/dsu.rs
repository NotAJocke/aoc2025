pub struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            // Path compression
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            // Union by size
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
        }
    }
}
