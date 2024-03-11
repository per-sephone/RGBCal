# rgbcal: RGB LED calibration tool

Nora Luna

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is _mostly_ finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring

Connect the RGB LED to the MB2 as follows:

- Red to P9 (GPIO1)
- Green to P8 (GPIO2)
- Blue to P16 (GPIO3)
- Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

- Pin 1 to Gnd
- Pin 2 to P2
- Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue.)

- No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
- A button held: Change the blue level from off to on over
  16 steps.
- B button held: Change the green level from off to on over
  16 steps.
- A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

## How it went

It took me a little longer than anticipated to fully understand all the moving pieces of the code. It took several hours of reviewing code and experimenting with the hardware and making small adjustments to see what happened. I had a difficult time coming up with a plan of action. I usually come up with a small outline of what I need to do: what steps, and in what order. Since I knew we had to pass the frame rate between the two crates, I initally just thought I could create a frame_rate variable in main to use between the two crates. This, obviously, did not end up working as expected. Since this was a road block, I decided to integrate the button functionality.

I figured the buttons would be straight forward, and I moved the `blue` button skeleton code into it's own function, passing in the correct index depending on the which LED was getting updated. Then in `run()` I made if-else statements to check each button press. Initially my button presses were not working correctly and this required more study of the code. I figured out that the skeleton code had a loop that continously updated the `blue` LED which was blocking from further scanning of the buttons. I needed to put the looping behavior in the correct place, instead of looping the knob measurements, I needed to continously scan the buttons then update measurement after. So I ended up with a loop inside of `run()` that continously scans for button presses. If the buttons are pressed, it will measure the knob for the appropriate color LED. With no buttons, it has a similar functionality but instead measures the frame rate.

Once I had this section working as I expected, I had to figure out how to pass the frame rate around. Upon finishing the above functionality, that gave me the hint that since the RGB levels were being passed around by mutex, that I also needed to pass the frame_rate by mutex. I created two functions in `main()` that "set" and "get" the frame_rate, similar to how they "set" and "get" the RBG levels. After that, there were only small updates needed in the `RGB` crate to get the current frame_rate and then calculate the tick_time. Pretty neat homework assignment, overall. 

## Measurements!!

According to my measurements we can get a nice white(ish) light at:
Red: 9
Green: 11
Blue: 3
Minimum Frame-Rate: 50