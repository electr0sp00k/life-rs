# life-rs
Teo Coffman, CS 510: Embedded Development with RUST

This Rust app for the BBC micro:bit v2 is an implementaion of
the game of life for 5x5 led display using the A and B buttons
to provide some interactivity.

## Program Specifications

* The program runs the game at 10 frames per second (updates once per 100ms).

* The program starts with a random board.

* While the A button is held, the board is re-randomized every frame.

* Otherwise, when the B button is not ignored and is pressed, the board is "complemented": every "on" cell is turned "off" and every "off" cell is turned "on". The B button is then   ignored for 5 frames (0.5s).

* Otherwise, if the program reaches a state where all cells on the board are off, the program waits 5 frames (0.5s). If it has not received a button press, it then starts with a new random board.

* Otherwise, if not paused, normal Life steps are taken.


## Development and Environment

All development, debugging, and testing was done on Windows 11. I did not run into any issues with using Windows as a development platform. Code was compiled and flashed using the cargo embed platform.

The development of this program was relatively simple after I modularized the development into steps, some of these steps were:

* Display the game_board state using the micro::bit display.show() function.

* Determine how to read the button state using board.buttons.button_b.is_low().unwrap()

* Determine how to keep track of 500ms before the B button compliment function is allowed again.

I am unhappy with my implementation of the counter to determine the delay between B button presses. This is somewhat documented in the comments of my code. To summarize the implementation, a counter is initialized, and every frame, that counter is incremented until the B button is pressed, at which time, it is reset to 0. If the B button is pressed before the counter reaches 5, the button press is ignored. I would rather implement this using a hardware clock that begins at runtime, and which the timestamp of the last B button press is stored at presstime, and when the button is pressed again, the current time is compared with the timestamp, and if the difference is less than 5 frames, than the press is ignored. I could not find documentation in the HAL section of the micro::bit crate to implement this, so I instead used the ugly brutelike counter implementation.

I also am unhappy with the way the display.show() function is the implementation for a delay, I would rather have the board display perpetualy until update, and use the  timer.delay_ms() function to create the delay for frames. I didn't find this in the rust microbit crate and since this implementation works, I decided it was fine.

One issue I ran into, was that initially I had the A and B button checks before the game of life was computed, which when the board was complemented, led to the display showing the turn after the complement, which drove me crazy for 15 mins until I realized I had made this mistake.

The other issue was that the else block that incremented the counter for the B button press, was initialy placed after the check for if the counter was greater than 5, which led to it only being incremented when the B button was pressed. By moving this to after the B button check, the counter incremented as intendended.

Other than these few issues, and a dependency issue with my Cargo.toml, I had almost no issues developing this program.

## License

This work is made available under the "MIT License". Please
see the file `LICENSE.txt` in this distribution for license
terms.

## Acknowledgements

Thanks to Bart Massey for providing the life.rs code (and several other example rust micro::bit programs) that runs game steps and does an empty state check. Thanks to the Micro::bit rust crates development team for creating a very easy interface for developing rust programs on the micro::bit.
