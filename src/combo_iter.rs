// Currently unusable, increases program time by ~100 ms
// prolly because of the cloning, need to figure out lifetime stuff


struct ComboIter {
    r: usize,
    current: Vec<usize>,
    max: Vec<usize>,
}

impl ComboIter {
    pub fn new() -> Self {
        let mut s = Self {
            r,
            current: Vec::from_iter((0..n).take(r)),
            max: Vec::from_iter((0..n).rev().take(r).rev()),
        };

        s.current[r - 1] -= 1;

        s
    }
}

impl Iterator for ComboIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.r;

        while i > 0 && self.current[i - 1] >= self.max[i - 1] {
            i -= 1;
        }

        if i == 0 {
            return None;
        }

        self.current[i - 1] += 1;

        if i == self.r {
            return Some(self.current.clone());
        }

        while i < self.r {
            self.current[i] = self.current[i - 1] + 1;
            i += 1;
        }

        Some(self.current.clone())
    }
}