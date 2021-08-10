pub struct Direction(isize, isize);

impl Direction {
    pub fn next(&self) -> Self {
        match (self.0, self.1) {
            (0, 1) => Self(1, 0),
            (1, 0) => Self(0, -1),
            (0, -1) => Self(-1, 0),
            _ => Self(0, 1),
        }
    }

    pub fn try_set(&mut self, i: usize, j: usize) -> (usize, usize) {
        (
            (i as isize + self.0) as usize,
            (j as isize + self.1) as usize,
        )
    }

    pub fn set(&mut self, i: &mut usize, j: &mut usize) {
        *i = (*i as isize + self.0) as usize;
        *j = (*j as isize + self.1) as usize;
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self(0, 1)
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut v = (0..size)
        .into_iter()
        .map(|_| {
            let v: Vec<u32> = vec![0; size as usize];
            v
        })
        .collect::<Vec<Vec<u32>>>();

    let mut i = 0usize;
    let mut j = 0usize;
    let mut direction = Direction::default();
    for n in 1..=(size * size) as usize {
        v[i][j] = n as u32;
        let (ii, jj) = direction.try_set(i, j);
        if let Some(value) = v.get(ii).and_then(|v| v.get(jj)) {
            if *value == 0 {
                direction.set(&mut i, &mut j);
                continue;
            }
        }
        direction = direction.next();
        direction.set(&mut i, &mut j);
    }
    v
}
