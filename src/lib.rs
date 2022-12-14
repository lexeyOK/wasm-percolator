mod dfs_connected_components;

use dfs_connected_components::FindComponentsDfs;
use wasm_bindgen::prelude::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render(seed: u32) -> Box<[u32]> {
    let (right, down) = init_random(seed, WIDTH, HEIGHT);

    let mut components = FindComponentsDfs::new(right, down, WIDTH, HEIGHT);
    components.find_connected_components();
    let field = components.get_connected_components();

    let mut buffer = vec![0; WIDTH * HEIGHT];

    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            buffer[x + y * HEIGHT] = xor_shift32(field[y][x] as u32 + seed) | 0xff000000;
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
