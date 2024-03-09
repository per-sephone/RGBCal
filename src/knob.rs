
/// This crate provides functionality for interfacing with a knob using an ADC (Analog to Digital Converter).
use crate::*;
/// Type alias for ADC (Analog to Digital Converter).
pub type Adc = saadc::Saadc<'static, 1>;
/// `Knob` represents a knob interfaced with an ADC.
pub struct Knob(Adc);

impl Knob {
    /// Initializes a new `Knob` instance.
    /// Calibrates the ADC and returns the `Knob` instance.
    /// # Arguments
    /// * `adc` - An `Adc` instance -- Analog to Digital Converter.
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }
    /// Measures the position of the knob.
    /// Samples the ADC and calculates the position of the knob.
    /// # Returns
    /// The value of the knob position.
    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
