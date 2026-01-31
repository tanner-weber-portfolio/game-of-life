/* File: main.rs
 *
 * Author: Tanner Weber, tannerw@pdx.edu
 *
 * Date: 2026
 */

//! John Conway's Game of Life on the Microbit V2

#![no_std]
#![no_main]

mod life;
use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use nanorand::Rng;
use panic_rtt_target as _;
use rtt_target::rprintln;

const FRAMETIME_MS: u32 = 100;
const RESTART_FRAMES_COUNT: u32 = 5;
const BUTTON_DELAY_FRAMES_COUNT: u32 = 5;

#[entry]
fn init() -> ! {
    rtt_target::rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let seed: u128 = 1;
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = get_random_led_board(seed);
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let mut button_b_delay: u32 = 0;

    loop {
        rprintln!("        Board {:?}", leds);

        if life::done(&leds) {
            rprintln!("DONE");
            timer.delay_ms(RESTART_FRAMES_COUNT * FRAMETIME_MS);
            leds = get_random_led_board(seed);
            continue;
        }
        if button_a.is_low().unwrap() {
            leds = get_random_led_board(seed);
        }
        if button_b.is_low().unwrap() {
            if button_b_delay == 0 {
                leds = flip_led_board(leds);
                rprintln!("FLIPPED Board {:?}", leds);
            }
            button_b_delay = BUTTON_DELAY_FRAMES_COUNT + 1;
        }
        button_b_delay = button_b_delay.saturating_sub(1);

        life::life(&mut leds);
        display.show(&mut timer, leds, FRAMETIME_MS);
        timer.delay_ms(FRAMETIME_MS);
    }
}

/// Returns a 5 x 5 array with each cell being 1 or 0.
fn get_random_led_board(seed: u128) -> [[u8; 5]; 5] {
    let mut rng = nanorand::Pcg64::new_seed(seed);
    let mut leds: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    for row in &mut leds {
        for cell in row {
            *cell = rng.generate_range(0_u8..2);
        }
    }

    leds
}

/// Turns each cell in a 5 x 5 from a 0 to 1 or vice versa.
fn flip_led_board(mut leds: [[u8; 5]; 5]) -> [[u8; 5]; 5] {
    for row in &mut leds {
        for cell in row {
            if *cell == 0 {
                *cell = 1;
            } else {
                *cell = 0;
            }
        }
    }

    leds
}
