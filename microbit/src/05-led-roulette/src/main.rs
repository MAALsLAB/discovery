#![deny(unsafe_code)]
#![no_main] // don't use standard 'main' instead user 'entry' attribute from 'cortex-m-rt'
#![no_std] // don't use 'std' crate, instead use the 'core' crate

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

const LED: [(usize, usize); 16] = [
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
    (1, 4), (2, 4), (3, 4), (4, 4), (4, 3),
    (4, 2), (4, 1), (4, 0), (3, 0), (2, 0),
    (1, 0)
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let time_delay: u32 = 30;

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut light_it_all = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut last_led = (0, 0);

    // infinite loop; just so we don't leave this stack frame
    loop {
        for current_led in LED.iter() {
            light_it_all[last_led.0][last_led.1] = 0;
            light_it_all[current_led.0][current_led.1] = 1;
            display.show(&mut timer, light_it_all, time_delay); // Show light_it_all for 1s
            last_led = *current_led;   
        }
    }
}
