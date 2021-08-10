use std::iter;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut direction = [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().cycle();
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let (mut x, mut y, mut n) = (-1, 0, 1..);
    iter::once(size).chain(
        (1..size).rev().flat_map(|x| iter::repeat(x).take(2))
    ).flat_map(|times| iter::repeat(direction.next().unwrap()).take(times as usize)).for_each(|(dy, dx)| {
        y += dy;
        x += dx;
        matrix[y as usize][x as usize] = n.next().unwrap();
    });

    matrix
}
