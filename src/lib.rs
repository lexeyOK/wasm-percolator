use disjoint_sets::UnionFind;
use std::cmp::{max, min};
use wasm_bindgen::prelude::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render(seed: u32) -> Box<[u32]> {
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let field = init_random(seed);
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            buffer[x + y * HEIGHT] =
                ((xor_shift32(field[y][x] as u32 + seed) >> 8) << 8 | 0xff).to_be();
        }
    }
    buffer.into_boxed_slice()
}

fn xor_shift32(state: u32) -> u32 {
    let mut x = state.wrapping_mul(0x2545F495);
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

fn init_random(seed: u32) -> Vec<Vec<usize>> {
    let mut right = vec![vec![false; WIDTH + 1]; HEIGHT + 1];
    let mut down = vec![vec![false; WIDTH + 1]; HEIGHT + 1];
    let mut state = seed;
    for x in 1..WIDTH {
        for y in 1..HEIGHT {
            state = xor_shift32(state);
            right[y][x] = state % 2 == 0;
            down[y][x] = (state >> 1) % 2 == 0;
        }
    }
    (1..=HEIGHT).for_each(|y| {
        right[y][WIDTH] = false;
    });
    (1..=WIDTH).for_each(|x| {
        down[HEIGHT][x] = false;
    });
    find_connected_components(right, down)
}

fn find_connected_components(right: Vec<Vec<bool>>, down: Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let mut field = vec![vec![0; WIDTH]; HEIGHT];
    if WIDTH == 0 || HEIGHT == 0 {
        return field;
    }
    let mut next_lable = 1;
    let mut eq_set: UnionFind<usize> = UnionFind::new(WIDTH * HEIGHT);
    for x in 1..=WIDTH {
        for y in 1..=HEIGHT {
            let (nodes, num) = {
                let mut temp = (0, 0);
                let mut num = 0;
                if down[y - 1][x] {
                    temp.0 = field[y - 2][x - 1];
                    num += 1;
                }
                if right[y][x - 1] {
                    temp.1 = field[y - 1][x - 2];
                    num += 1;
                }
                (temp, num)
            };
            match num {
                0 => {
                    field[y - 1][x - 1] = next_lable;
                    next_lable += 1;
                }
                1 => {
                    field[y - 1][x - 1] = max(nodes.0, nodes.1);
                }
                2 => {
                    field[y - 1][x - 1] = min(nodes.0, nodes.1);
                    eq_set.union(nodes.0, nodes.1);
                }
                _ => unreachable!(),
            }
        }
    }

    let mut output_lables = vec![0usize; WIDTH * HEIGHT];
    let mut count = 0;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let root = eq_set.find(field[y][x]);
            let output_lable = &mut output_lables[root];
            if *output_lable == 0 {
                *output_lable = count;
                count += 1;
            }
            field[y][x] = *output_lable;
        }
    }
    field
}
