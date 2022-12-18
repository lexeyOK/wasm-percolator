use Option::{None, Some};

pub(crate) struct FindComponentsDfs<const W: usize, const H: usize>
where
    [(); W + 1]: Sized,
    [(); H + 1]: Sized,
{
    right: [[bool; W + 1]; H + 1],
    down: [[bool; W + 1]; H + 1],
    pub components: [[usize; W]; H],
    used: [[bool; W]; H],
}
impl<const W: usize, const H: usize> FindComponentsDfs<W, H>
where
    [(); W + 1]: Sized,
    [(); H + 1]: Sized,
{
    pub fn new(right: [[bool; W + 1]; H + 1], down: [[bool; W + 1]; H + 1]) -> Self {
        Self {
            right,
            down,
            components: [[0; W]; H],
            used: [[false; W]; H],
        }
    }

    fn dfs(&mut self, x: usize, y: usize, component_number: usize) {
        assert!(x > 0);
        assert!(y > 0);
        self.used[y - 1][x - 1] = true;
        self.components[y - 1][x - 1] = component_number;
        for vert in self.get_neighbors(x, y) {
            if let Some((x_u, y_u)) = vert {
                if !self.used[y_u - 1][x_u - 1] {
                    self.dfs(x_u, y_u, component_number);
                }
            }
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
        let mut verts = [None; 4];
        if self.down[y - 1][x] {
            verts[0] = Some((x, y - 1));
        }
        if self.down[y][x] {
            verts[1] = Some((x, y + 1));
        }
        if self.right[y][x - 1] {
            verts[2] = Some((x - 1, y));
        }
        if self.right[y][x] {
            verts[3] = Some((x + 1, y));
        }
        verts
    }

    pub fn find_connected_components(&mut self) {
        let mut component_number = 1;
        for x in 1..=W {
            for y in 1..=H {
                if !self.used[y - 1][x - 1] {
                    self.dfs(x, y, component_number);
                    component_number += 1;
                }
            }
        }
    }
    pub fn get_connected_components(&self) -> [[usize; W]; H] {
        self.components
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn xor_shift32(state: u32) -> u32 {
        let mut x = state.wrapping_mul(0x2545F495);
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }

    fn init_random<const W: usize, const H: usize>(
        seed: u32,
    ) -> ([[bool; W + 1]; H + 1], [[bool; W + 1]; H + 1])
    where
        [(); W]: Sized,
        [(); H]: Sized,
    {
        let mut right = [[false; W + 1]; H + 1];
        let mut down = [[false; W + 1]; H + 1];

        let mut state = seed;

        for x in 1..W {
            for y in 1..H {
                state = xor_shift32(state);
                right[y][x] = state % 2 == 0;
                down[y][x] = (state >> 1) % 2 == 0;
            }
        }
        (right, down)
    }

    #[test]
    fn simple_test() {
        let mut dfs = FindComponentsDfs::new(
            [
                [false, false, false],
                [false, true, false],
                [false, false, false],
            ],
            [
                [false, false, false],
                [false, true, false],
                [false, false, false],
            ],
        );
        dfs.find_connected_components();
        assert_eq!(dfs.components, [[1, 1], [1, 2]]);
    }
    #[test]
    fn other_test() {
        const WIDTH: usize = 140;
        let (right, down) = init_random::<WIDTH, WIDTH>(12);
        let mut components = FindComponentsDfs::<WIDTH, WIDTH>::new(right, down);
        components.find_connected_components();
    }
}
