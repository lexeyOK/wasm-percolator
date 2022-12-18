#![no_std]
#![feature(generic_const_exprs)]

#[panic_handler]
fn panic_impl(some:&PanicInfo) ->!{
    loop {}
}

mod dfs_connected_components;

use core::panic::PanicInfo;

use dfs_connected_components::FindComponentsDfs;

const WIDTH: usize = 140;
const HEIGHT: usize = 140;
#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[no_mangle]
pub fn render(seed: u32) {
    let (right, down) = init_random::<WIDTH, HEIGHT>(seed);

    let mut components = FindComponentsDfs::<WIDTH, HEIGHT>::new(right, down);
    components.find_connected_components();
    let field = components.get_connected_components();

    let buffer = unsafe { &mut BUFFER };

    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            buffer[x + y * HEIGHT] = xor_shift32(field[y][x] as u32 + seed) | 0xff000000;
        }
    }
}

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
