pub struct FindComponentsDfs {
    right: Vec<Vec<bool>>,
    down: Vec<Vec<bool>>,
    components: Vec<Vec<usize>>,
    used: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl FindComponentsDfs {
    pub fn new(right: Vec<Vec<bool>>, down: Vec<Vec<bool>>, width: usize, height: usize) -> Self {
        // TODO: validate right down as rectangular matrices
        // probably with assertions
        Self {
            right,
            down,
            components: vec![vec![0; width]; height],
            used: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    fn dfs(&mut self, x: usize, y: usize, component_number: usize) {
        assert!(x > 0);
        assert!(y > 0);

        let mut stack = vec![(x, y)];

        while let Some((x, y)) = stack.pop() {
            self.used[y - 1][x - 1] = true;
            self.components[y - 1][x - 1] = component_number;

            for (x_u, y_u) in self.get_neighbors(x, y) {
                if !self.used[y_u - 1][x_u - 1] {
                    stack.push((x_u, y_u));
                }
            }
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut verts = Vec::with_capacity(4);
        if self.down[y - 1][x] {
            verts.push((x, y - 1));
        }
        if self.down[y][x] {
            verts.push((x, y + 1));
        }
        if self.right[y][x - 1] {
            verts.push((x - 1, y));
        }
        if self.right[y][x] {
            verts.push((x + 1, y));
        }
        verts
    }

    pub fn find_connected_components(&mut self) {
        let mut component_number = 1;
        for x in 1..=self.width {
            for y in 1..=self.height {
                if !self.used[y - 1][x - 1] {
                    self.dfs(x, y, component_number);
                    component_number += 1;
                }
            }
        }
    }
    pub fn get_connected_components(self) -> Vec<Vec<usize>> {
        self.components
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init_random(seed: u32, width: usize, height: usize) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
        let mut right = vec![vec![false; width + 1]; height + 1];
        let mut down = vec![vec![false; width + 1]; height + 1];
        let mut state = seed;
        for x in 1..width {
            for y in 1..height {
                state = xor_shift32(state);
                right[y][x] = state % 2 == 0;
                down[y][x] = (state >> 1) % 2 == 0;
            }
        }
        (right, down)
    }
    fn xor_shift32(state: u32) -> u32 {
        let mut x = state.wrapping_mul(0x2545F495);
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }
    #[test]
    fn simple_test() {
        let mut dfs = FindComponentsDfs::new(
            vec![
                vec![false, false, false],
                vec![false, true, false],
                vec![false, false, false],
            ],
            vec![
                vec![false, false, false],
                vec![false, true, false],
                vec![false, false, false],
            ],
            2,
            2,
        );
        dfs.find_connected_components();
        assert_eq!(dfs.components, vec![vec![1, 1], vec![1, 2]]);
    }
    #[test]
    fn other_test() {
        let (right, down) = init_random(12, 5, 4);
        let mut components = FindComponentsDfs::new(right, down, 5, 4);
        components.find_connected_components();
        assert_eq!(
            components.components,
            vec![
                vec![1, 1, 1, 4, 6],
                vec![1, 1, 1, 1, 1],
                vec![2, 2, 1, 1, 7],
                vec![3, 2, 1, 5, 8],
            ]
        );
    }
}
