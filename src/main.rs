#![no_std]
#![no_main]

mod life;
use life::*;
use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal::prelude::*;
use microbit::hal::Timer;
use nanorand::{pcg64::Pcg64, Rng};
use panic_halt as _;

// Macro that places random 0-1 values in the gameboard matrix
macro_rules! randomize_game_board {
    ($game_board:expr, $rng:expr) => {
        for row in $game_board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = $rng.generate_range(0..2);
            }
        }
    };
}

#[entry]
fn init() -> ! {
    let board = Board::take().unwrap();
    let mut display = microbit::display::blocking::Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = Pcg64::new_seed(1);
    let mut game_board = [[0u8; 5]; 5];

    // Initialize the game board with random values
    randomize_game_board!(game_board, rng);

    let mut counter = 0u32;

    loop {

        // If all cells are off
        if done(&game_board) {
            
            // Delay for 5 frames
            timer.delay_ms(500_u16);
            
            // Randomize board
            randomize_game_board!(game_board, rng);

        } else {
            // Perform the Game of Life turn
            life(&mut game_board);
        }

        // Reset the board if button A is pressed
        if board.buttons.button_a.is_low().unwrap() {
            randomize_game_board!(game_board, rng);
        }

        // Complement the board if button B is pressed; else increment the counter.
        if board.buttons.button_b.is_low().unwrap() {
            // This counter system seems stupid, wondering if I can return the systime or something
            // from the HAL and when button pressed, check if the current time is 500ms more than
            // the last time, therefore remoiving the need to increment the counter every frame

            // If 5 frames has passed since last press
            if counter > 5 {
                for row in game_board.iter_mut() {
                    for cell in row.iter_mut() {
                        // Complement the board (panic if neither 0 or 1; debug purposes)
                        *cell = match *cell {
                            0 => 1,
                            1 => 0,
                            _ => panic!("Unexpected value in game_board!"),
                        };
                    }
                }
                // Reset counter to 0; preventing B presses more than once per 5 frames
                counter = 0;
            } 
        } else {
            // Why doesn't Rust have ++ ? TODO: write Rust++
            counter += 1;
        }
        // Display the game board, with a delay of 100ms (is this how it should work?)
        // Can I display without a delay and continue running the rest of the code?
        display.show(&mut timer, game_board, 100_u32);
    }
}
