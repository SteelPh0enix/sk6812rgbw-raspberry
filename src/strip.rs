use crate::led::Led;
use palette::{Gradient, LinSrgb, Srgb};
pub use rppal::spi::{Bus, SlaveSelect};
use rppal::spi::{Mode, Spi};
use std::{
    error::Error,
    ops::{ShlAssign, ShrAssign},
    thread,
    time::Duration,
};

const SPI_FREQUENCY: u32 = 6_400_000;

/// Structure representing whole SK6812RGBW strip.
/// Should be compatible with other similar LED's, but they would likely require a different bit ordering
pub struct Strip {
    spi: Spi,
    pub leds: Vec<Led>,
}

impl Strip {
    /// Create new SK6812RGBW strip
    /// Since rppal library requires slave-select pin to initalize SPI, by default SS0 is selected. It's not used to drive LEDs, so it's a wasted pin.
    /// If you want to select other pin, use `new_with_custom_ss` method.
    pub fn new(bus: Bus, amount_of_leds: usize) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            spi: Spi::new(bus, SlaveSelect::Ss0, SPI_FREQUENCY, Mode::Mode0)?,
            leds: vec![Led::new(); amount_of_leds],
        })
    }

    /// Create new SK6812RGBW strip with custom slave-select pin
    /// If you want to use SS0 for different purposes, you can waste another pin with this function instead.
    pub fn new_with_custom_ss(
        bus: Bus,
        amount_of_leds: usize,
        slave_select: SlaveSelect,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            spi: Spi::new(bus, slave_select, SPI_FREQUENCY, Mode::Mode0)?,
            leds: vec![Led::new(); amount_of_leds],
        })
    }

    /// Set the color of all LEDs in the strip at once
    pub fn fill(&mut self, led: Led) {
        self.leds.fill(led);
    }

    // Turn off all the LEDs
    pub fn clear(&mut self) {
        self.leds.fill(Led::new());
    }

    pub fn set_gradient(&mut self, gradient: Gradient<LinSrgb>) {
        gradient
            .take(self.leds.len())
            .zip(&mut self.leds)
            .for_each(|(color, led)| {
                *led = Srgb::from_linear(color).into();
            });
    }

    // "Rotate" LEDs to the beginning of the strip - move the first LED color to the end, 2nd to the first, and so on.
    pub fn shift_left(&mut self, count: usize) {
        self.leds.rotate_left(count);
    }

    pub fn shift_right(&mut self, count: usize) {
        self.leds.rotate_right(count);
    }

    /// Call this to send the data from `leds` to the strip
    /// This function will block the thread for ~80us after sending the data,
    /// which is caused by strip comms protocol requirements.
    ///
    /// If you're getting an error, telling you that the message is too long - increase the SPI transfer size in `/boot/cmdline.txt`.
    /// To do so, add `spidev.bufsiz=65535` to the first line of this file. I added it right before `rootwait`, but placement shouldn't matter.
    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let led_data: Vec<u8> = self.get_led_data().collect();
        self.spi.write(&led_data)?;
        thread::sleep(Duration::from_micros(80));

        Ok(())
    }

    fn get_led_data(&self) -> impl Iterator<Item = u8> + '_ {
        self.leds.iter().flat_map(|led| led.to_raw_led_bytes())
    }
}

impl ShrAssign<usize> for Strip {
    fn shr_assign(&mut self, rhs: usize) {
        self.shift_right(rhs);
    }
}

impl ShlAssign<usize> for Strip {
    fn shl_assign(&mut self, rhs: usize) {
        self.shift_left(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_strip() -> Strip {
        Strip::new(Bus::Spi0, 144).unwrap()
    }

    #[test]
    fn test_setting_color() {
        let led: Led = [100, 0, 0].into();
        let mut strip = make_strip();

        strip.fill(led);

        strip.leds.iter().for_each(|strip_led| {
            assert_eq!(*strip_led, led);
        })
    }

    #[test]
    fn test_clearing() {
        let led: Led = [100, 0, 0].into();
        let mut strip = make_strip();

        strip.fill(led);
        strip.clear();

        strip.leds.iter().for_each(|strip_led| {
            assert_eq!(*strip_led, Led::new());
        })
    }

    #[test]
    fn test_shift_right() {
        let mut strip = Strip::new(Bus::Spi0, 5).unwrap();

        strip.leds[0].r = 1;
        strip.leds[1].r = 2;
        strip.leds[2].r = 3;
        strip.leds[3].r = 4;
        strip.leds[4].r = 5;

        strip >>= 1;

        assert_eq!(strip.leds[0].r, 5);
        assert_eq!(strip.leds[1].r, 1);
        assert_eq!(strip.leds[2].r, 2);
        assert_eq!(strip.leds[3].r, 3);
        assert_eq!(strip.leds[4].r, 4);
    }

    #[test]
    fn test_shift_left() {
        let mut strip = Strip::new(Bus::Spi0, 5).unwrap();

        strip.leds[0].r = 1;
        strip.leds[1].r = 2;
        strip.leds[2].r = 3;
        strip.leds[3].r = 4;
        strip.leds[4].r = 5;

        strip <<= 1;

        assert_eq!(strip.leds[0].r, 2);
        assert_eq!(strip.leds[1].r, 3);
        assert_eq!(strip.leds[2].r, 4);
        assert_eq!(strip.leds[3].r, 5);
        assert_eq!(strip.leds[4].r, 1);
    }
}
