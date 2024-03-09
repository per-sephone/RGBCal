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
}

/// the default state for each color of light.
impl Default for UiState {
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
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

impl Ui {
    /// Creates a new Ui instance.
    /// # Arguments
    /// * `knob` - the connected knob on the breadboard.
    /// * `_button_a` - Button A on the microbit.
    /// * `_button_b` - Button B on the microbit.
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    /// Runs the UI, continuously updating RGB levels based on knob input.
    pub async fn run(&mut self) -> ! {
        self.state.levels[2] = self.knob.measure().await;
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            let level = self.knob.measure().await;
            if level != self.state.levels[2] {
                self.state.levels[2] = level;
                self.state.show();
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }
            Timer::after_millis(50).await;
        }
    }
}
