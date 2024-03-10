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
    /// Displays the current RGB levels and frame rate.
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
    fn new(rate: u64) -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: rate,
        }
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
    /// * `_button_a` - Button A on the microbit.
    /// * `_button_b` - Button B on the microbit.
    pub fn new(knob: Knob, button_a: Button, button_b: Button, frame_rate: u64) -> Self {
        Self {
            knob,
            button_a,
            button_b,
            //state: UiState::default(),
            state: UiState::new(frame_rate),
        }
    }

    /// Runs the UI, continuously updating RGB levels based on knob input.
    pub async fn run(&mut self) -> ! {
        loop {
            if self.button_a.is_low() && self.button_b.is_low() {
                rprintln!("RED LED");
                self.change_color_measurement(0).await
            }
            else if self.button_a.is_low() {
                rprintln!("BLUE LED");
                self.change_color_measurement(2).await
            }
            else if self.button_b.is_low() {
                rprintln!("GREEN LED");
                self.change_color_measurement(1).await
            }
            else {
                let level = self.knob.measure().await as u64 * 10 + 10;
                if level != self.state.frame_rate {
                    self.state.frame_rate = level;
                    self.state.show();
                    set_frame_rate(|fr | {
                        *fr = self.state.frame_rate;
                    }).await;
                }
                Timer::after_millis(50).await;
            }
        }

    }

    pub async fn change_color_measurement(&mut self, position: usize) -> () {
        self.state.levels[position] = self.knob.measure().await;
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        Timer::after_millis(50).await;
    }

}
