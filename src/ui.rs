/// This crate provides a user interface (UI) module for interacting with the breadboard
/// containing the microbit v2, an RGB LED, and a knob.
use crate::*;
/// Represents the state of the UI.
/// the levels of each color light,
/// and the frame rate
struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    /// Prints the current RGB levels and frame rate to the screen.
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

/// Updates the frame rate
impl Default for UiState {
    /// the default state for each color of light.
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

/// A `Ui` struct contains the knob connected through the breadboard,
/// Button A on the microbit, and
/// Button B on the microbit, as well as
/// the state of the UI
pub struct Ui {
    knob: Knob,
    button_a: Button,
    button_b: Button,
    state: UiState,
}

impl Ui {
    /// Creates a new Ui instance.
    /// # Arguments
    /// * `knob` - the connected knob on the breadboard.
    /// * `button_a` - Button A on the microbit.
    /// * `button_b` - Button B on the microbit.
    pub fn new(knob: Knob, button_a: Button, button_b: Button) -> Self {
        Self {
            knob,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    /// Runs the UI, continuously updating RGB levels and frame rate
    /// based on knob input and buttons pressed.
    pub async fn run(&mut self) -> ! {
        loop {
            //RED LED
            if self.button_a.is_low() && self.button_b.is_low() {
                self.change_color_measurement(0).await
            }
            //BLUE LED
            else if self.button_a.is_low() {
                self.change_color_measurement(2).await
            }
            //GREEN LED
            else if self.button_b.is_low() {
                self.change_color_measurement(1).await
            } else {
                // no buttons pressed, controls the overall frame rate
                self.state.frame_rate = self.knob.measure().await as u64 * 10 + 10;
                set_frame_rate(|fr| {
                    *fr = self.state.frame_rate;
                })
                .await;
                self.state.show();
                Timer::after_millis(50).await;
            }
        }
    }

    /// Checks the level for the given position in the levels array and updates it based on the knob measurement.
    /// Sets the level using a lock and setter method, shows the update level.
    /// # Arguments
    /// * `position` - the index in the levels array (0 for red, 1 for green, 2 for blue)
    pub async fn change_color_measurement(&mut self, position: usize) {
        self.state.levels[position] = self.knob.measure().await;
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        Timer::after_millis(50).await;
    }
}
