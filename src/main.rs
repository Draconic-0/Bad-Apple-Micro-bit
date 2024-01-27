#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use rtt_target::rprint;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let str = include_bytes!("BadApple");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut frame = 0;
    loop {
        let light = make_frame(str, frame);
        // print_disp for debugging purposes
        // print_disp(&light);
        display.show(&mut timer, light, 33);
        if frame < 6571 {
            frame += 1;
        }
        display.clear();
    }
}
fn make_frame(vd: &[u8], x: usize) -> [[u8; 5]; 5] {
    let mut frame: [[u8; 5]; 5] = [[0; 5]; 5];
    let mut i: usize = 0;
    let mut j: usize;
    let mut b: u8;
    while i < 5 {
        j = 0;
        while j < 5 {
            b = vd[x * 25 + 5 * i + j]/42;
            frame[i][j] = b;
            j += 1;
        }
        i += 1
    }
    return frame;
}
// fn print_disp(disp: &[[u8; 5]; 5]) -> () {
//     let mut i: usize = 0;
//     let mut j: usize = 0;
//     while i < 5 {
//         j = 0;
//         while j < 5 {
//             rprint!("{} ", disp[i][j]);
//             j += 1;
//         }
//         rprint!("\n");
//         i += 1
//     }
// }
