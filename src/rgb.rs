/// This crate provides embedded functionality for controlling RGB LEDs with Pulse Width Modulation on the microbit v2.
use crate::*;

/// Type alias for RGB pins connected to the LED light.
pub type RgbPins = [Output<'static, AnyPin>; 3];

/// Struct representing an RGB LED Pins:
/// Contains the pins for red, ground, green and blue,
/// the levels for each pin
/// and the time for each "tick"
pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

impl Rgb {
    /// Calculates the time for each "tick" based on the passed in frame rate.
    /// # Arguments
    /// * `frame_rate` - the frame_rate for each color display.
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    /// Initializes the RGB LED controller with the specified RGB pins and frame rate.
    /// Gets the current frame rate and calculates the tick time based on the current frame rate.
    /// # Arguments
    /// * `rgb` - An array of output pins representing each color pin on the LED.
    pub async fn new(rgb: RgbPins) -> Self {
        let frame_rate = get_frame_rate().await;
        let tick_time = Self::frame_tick_time(frame_rate);
        rprintln!("TickTime: {}", tick_time);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Executes a step in the RGB color switching.
    /// Controls the Pulse Width Modulation of each color based on the current level.
    /// # Arguments
    /// * `led` - The array that contains an index for each color (0 for red, 1 for green, 2 for blue).
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }
    /// Runs the RGB loop.
    /// Continuously updates the RGB LED levels and executes each color step.
    /// Continuously updates the frame rate and calculates the tick time.
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;

            let frame_rate = get_frame_rate().await;
            self.tick_time = Self::frame_tick_time(frame_rate);

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
