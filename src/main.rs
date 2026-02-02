/* File: main.rs
 * Author: Tanner Weber, tannerw@pdx.edu
 * Date: 2 February 2026
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
const FIRST_ROUND_SEED: u128 = 1;

#[entry]
fn init() -> ! {
    rtt_target::rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut rng = microbit::hal::rng::Rng::new(board.RNG);
    let mut rng_buffer: [u8; 16] = [0; 16];
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = get_random_led_board(FIRST_ROUND_SEED);
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let mut button_b_delay: u32 = 0;

    loop {
        rng.random(&mut rng_buffer);
        let rng_num = u128::from_le_bytes(rng_buffer);

        if life::done(&leds) {
            rprintln!("DONE");
            timer.delay_ms(RESTART_FRAMES_COUNT * FRAMETIME_MS);
            leds = get_random_led_board(rng_num);
            continue;
        }
        if button_a.is_low().unwrap() {
            rprintln!("A Pressed");
            leds = get_random_led_board(rng_num);
        }
        if button_b.is_low().unwrap() {
            rprintln!("B Pressed");
            if button_b_delay == 0 {
                leds = flip_led_board(leds);
                rprintln!("FLIPPED Board {:?}", leds);
                button_b_delay = BUTTON_DELAY_FRAMES_COUNT + 1;
            }
        }
        button_b_delay = button_b_delay.saturating_sub(1);

        rprintln!("        Board {:?}", leds);
        display.show(&mut timer, leds, FRAMETIME_MS);
        life::life(&mut leds);
        timer.delay_ms(FRAMETIME_MS);
    }
}

/// Returns a 5x5 array with each cell being 1 or 0 randomly.
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

    rprintln!("New board generated");
    leds
}

/// Turns each cell in a 5x5 array from a 0 to 1 or vice versa.
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

    rprintln!("Leds flipped");
    leds
}
