use disjoint_sets::UnionFind;
use std::cmp::{max, min};
use wasm_bindgen::prelude::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

#[wasm_bindgen(inline_js = "export function gen_bool(prob){return Math.random()>prob}")]
extern "C" {
    fn gen_bool(prob: f32) -> bool;
}

#[wasm_bindgen]
pub fn render() -> Box<[u32]> {
    console_error_panic_hook::set_once();
    let mut buffer = vec![0;WIDTH*HEIGHT];
    let field = init_random();
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            let val = ((field[y][x]) % 256) as u32;
            buffer[x + y * HEIGHT] =
                ((13 * val) * 0x10000 + (17 * val) * 0x100 + (15 * val)) * 0x100 + 0xff;
        }
    }
    buffer.into_boxed_slice()
}

fn init_random() -> Vec<Vec<usize>> {
    let mut right = vec![vec![false; WIDTH + 1]; HEIGHT + 1];
    let mut down = vec![vec![false; WIDTH + 1]; HEIGHT + 1];
    for x in 1..WIDTH {
        for y in 1..HEIGHT {
            right[y][x] = gen_bool(0.5);
            down[y][x] = gen_bool(0.5);
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
        (0..HEIGHT).for_each(|y| {
            let label = field[y][x];
            let root = eq_set.find(label);
            let mut output_lable = output_lables[root];
            if output_lable < 1 {
                output_lable = count;
                count += 1;
            }
            output_lables[root] = output_lable;
            field[y][x] = output_lable;
        });
    }
    field
}
