pub use rppal::spi::{Bus, SlaveSelect};

use bitvec::prelude::*;
use rppal::spi::{Mode, Spi};
use std::error::Error;
use std::thread;
use std::time::Duration;

/// High bit (logical 1) representation for SPI
const BIT_HIGH: u8 = 0b11110000;
/// Low bit (logical 0) representation for SPI
const BIT_LOW: u8 = 0b11000000;

const SPI_FREQUENCY: u32 = 6_400_000;

/// Structure representing a single LED
#[derive(Clone, Copy, Debug)]
pub struct Led {
    r: u8,
    g: u8,
    b: u8,
    w: u8,
}

/// Structure representing whole SK6812RGBW strip.
/// Should be compatible with other similar LED's, but they would likely require a different bit ordering
pub struct SK6812RGBWStrip {
    spi: Spi,
    pub leds: Vec<Led>,
}

impl Led {
    fn new() -> Self {
        Led {
            r: 0,
            g: 0,
            b: 0,
            w: 0,
        }
    }

    fn to_sk6812_byte_vec(&self) -> Vec<u8> {
        [self.g, self.r, self.b, self.w]
            .view_bits::<Msb0>()
            .iter()
            .map(|bit| match *bit {
                true => BIT_HIGH,
                false => BIT_LOW,
            })
            .collect()
    }

    fn to_rgbw(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.w]
    }
}

impl SK6812RGBWStrip {
    /// Create new SK6812RGBW strip
    /// Since rppal library requires slave-select pin to initalize SPI, by default SS0 is selected. It's not used to drive LED's, so it's a wasted pin.
    /// If you want to select other pin, use `new_custom_ss` method.
    pub fn new(bus: Bus, amount_of_leds: usize) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            spi: Spi::new(bus, SlaveSelect::Ss0, SPI_FREQUENCY, Mode::Mode0)?,
            leds: vec![
                Led {
                    r: 0,
                    g: 0,
                    b: 0,
                    w: 0
                };
                amount_of_leds
            ],
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
    pub fn set_color(&mut self, led: &Led) {
        self.leds.fill(*led);
    }

    pub fn clear(&mut self) {
        self.leds.fill(Led::new());
    }

    /// Call this to send the data from `leds` to the strip
    /// This function will block the thread for ~80us after sending the data,
    /// which is caused by strip comms protocol requirements.
    ///
    /// If you're getting an error, telling you that the message is too long - increase the SPI transfer size in `/boot/cmdline.txt`.
    /// To do so, add `spidev.bufsiz=65535` to the first line of this file. I added it right before `rootwait`, but placement shouldn't matter.
    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let led_data: Vec<u8> = self.get_led_data();
        self.spi.write(&led_data)?;
        thread::sleep(Duration::from_micros(80));

        Ok(())
    }

    fn get_led_data(&self) -> Vec<u8> {
        self.leds
            .iter()
            .flat_map(|led| led.to_sk6812_byte_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LED_AMOUNT: usize = 144;

    #[test]
    fn test_led_to_byte_vec_conversion() {
        let led = Led {
            r: 0xAA,
            g: 0x00,
            b: 0xFF,
            w: 0x33,
        };
        let led_sk_bytes = led.to_sk6812_byte_vec();

        assert_eq!(led_sk_bytes.len(), 32);

        // Expected byte array should be in GRBW format
        assert_eq!(
            led_sk_bytes,
            [
                BIT_LOW, BIT_LOW, BIT_LOW, BIT_LOW, BIT_LOW, BIT_LOW, BIT_LOW, BIT_LOW, BIT_HIGH,
                BIT_LOW, BIT_HIGH, BIT_LOW, BIT_HIGH, BIT_LOW, BIT_HIGH, BIT_LOW, BIT_HIGH,
                BIT_HIGH, BIT_HIGH, BIT_HIGH, BIT_HIGH, BIT_HIGH, BIT_HIGH, BIT_HIGH, BIT_LOW,
                BIT_LOW, BIT_HIGH, BIT_HIGH, BIT_LOW, BIT_LOW, BIT_HIGH, BIT_HIGH
            ]
        );
    }

    #[test]
    fn test_led_to_byte_array_conversion() {
        assert_eq!(
            Led {
                r: 10,
                g: 20,
                b: 30,
                w: 40
            }
            .to_rgbw(),
            [10, 20, 30, 40]
        );
    }

    #[test]
    fn test_strip_single_color_fill() -> Result<(), Box<dyn Error>> {
        let mut strip = SK6812RGBWStrip::new(Bus::Spi0, LED_AMOUNT)?;

        strip.set_color(&Led {
            r: 100,
            g: 0,
            b: 0,
            w: 0,
        });
        strip.update()?;

        Ok(())
    }

    #[test]
    #[ignore]
    fn show_all_colors() -> Result<(), Box<dyn Error>> {
        const COLOR_DELAY: Duration = Duration::from_millis(500);

        let mut strip = SK6812RGBWStrip::new(Bus::Spi0, LED_AMOUNT)?;
        strip.set_color(&Led {
            r: 100,
            g: 0,
            b: 0,
            w: 0,
        });
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);

        strip.set_color(&Led {
            r: 0,
            g: 100,
            b: 0,
            w: 0,
        });
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);

        strip.set_color(&Led {
            r: 0,
            g: 0,
            b: 100,
            w: 0,
        });
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);

        strip.set_color(&Led {
            r: 100,
            g: 100,
            b: 0,
            w: 0,
        });
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);

        strip.set_color(&Led {
            r: 100,
            g: 0,
            b: 100,
            w: 0,
        });
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);

        strip.set_color(&Led {
            r: 0,
            g: 100,
            b: 100,
            w: 0,
        });
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);

        strip.set_color(&Led {
            r: 100,
            g: 100,
            b: 100,
            w: 0,
        });
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);

        strip.set_color(&Led {
            r: 0,
            g: 0,
            b: 0,
            w: 100,
        });
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);

        strip.set_color(&Led {
            r: 0,
            g: 0,
            b: 0,
            w: 0,
        });
        strip.update().unwrap();

        Ok(())
    }

    #[test]
    fn test_strip_clearing() -> Result<(), Box<dyn Error>> {
        let mut strip = SK6812RGBWStrip::new(Bus::Spi0, LED_AMOUNT)?;

        strip.clear();
        strip.update().unwrap();

        Ok(())
    }
}
