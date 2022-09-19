use bitvec::prelude::*;
use palette::Srgb;

/// High bit (logical 1) representation for SPI
const BIT_HIGH: u8 = 0b11110000;
/// Low bit (logical 0) representation for SPI
const BIT_LOW: u8 = 0b11000000;

/// Structure representing a single RGBW LED
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Led {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub w: u8,
}

impl Led {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_rgb_array(data: [u8; 3]) -> Self {
        data.into()
    }

    pub fn from_rgbw_array(data: [u8; 4]) -> Self {
        data.into()
    }

    pub fn to_rgbw_array(self) -> [u8; 4] {
        self.into()
    }

    pub fn to_rgb_array(self) -> [u8; 3] {
        self.into()
    }

    /// Converts the instance of this struct to SK6812-compatible byte array for SPI.
    /// Don't use in your own code, unless you know what you're doing.
    pub fn to_sk6812_bytes(&self) -> Vec<u8> {
        [self.g, self.r, self.b, self.w]
            .view_bits::<Msb0>()
            .iter()
            .map(|bit| match *bit {
                true => BIT_HIGH,
                false => BIT_LOW,
            })
            .collect()
    }
}

impl Into<[u8; 3]> for Led {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[u8; 4]> for Led {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.w]
    }
}

impl From<[u8; 3]> for Led {
    fn from(colors: [u8; 3]) -> Self {
        Led {
            r: colors[0],
            g: colors[1],
            b: colors[2],
            w: 0,
        }
    }
}

impl From<[u8; 4]> for Led {
    fn from(colors: [u8; 4]) -> Self {
        Led {
            r: colors[0],
            g: colors[1],
            b: colors[2],
            w: colors[3],
        }
    }
}

impl Into<Srgb> for Led {
    fn into(self) -> Srgb {
        Srgb::from_components((
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
        ))
    }
}

impl From<Srgb> for Led {
    fn from(color: Srgb) -> Self {
        [
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
        ]
        .into()
    }
}

#[cfg(test)]
mod tests {
    use palette::Srgb;

    use super::*;

    #[test]
    fn test_led_to_byte_vec_conversion() {
        let led = Led {
            r: 0xAA,
            g: 0x00,
            b: 0xFF,
            w: 0x33,
        };
        let led_sk_bytes = led.to_sk6812_bytes();

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
        let led = Led {
            r: 10,
            g: 20,
            b: 30,
            w: 40,
        };

        assert_eq!(led.to_rgbw_array(), [10, 20, 30, 40]);
        assert_eq!(led.to_rgb_array(), [10, 20, 30]);
    }

    #[test]
    fn test_led_from_array_creation() {
        let led_rgb = Led::from_rgb_array([10, 20, 30]);
        let led_rgbw = Led::from_rgbw_array([10, 20, 30, 40]);

        assert_eq!(led_rgb.to_rgb_array(), [10, 20, 30]);
        assert_eq!(led_rgbw.to_rgbw_array(), [10, 20, 30, 40]);
    }

    #[test]
    fn test_pixel_implementation_create_from_raw_data() {
        let pixel_raw_rgbw_data = [10, 20, 30, 40];
        let pixel_rgbw = Led::from_rgbw_array(pixel_raw_rgbw_data);
        let led_rgbw: Led = pixel_raw_rgbw_data.into();

        assert_eq!(led_rgbw, pixel_rgbw);
    }

    #[test]
    fn test_pixel_implementation_create_srgb_pixel_from_led() {
        let led: Led = [51, 102, 204].into();
        let pixel: Srgb = led.into();

        assert_eq!(pixel.red, 0.2);
        assert_eq!(pixel.green, 0.4);
        assert_eq!(pixel.blue, 0.8);
    }

    #[test]
    fn test_pixel_implementation_create_led_from_srgb_pixel() {
        let pixel = Srgb::from_components((0.2, 0.4, 0.8));
        let led: Led = pixel.into();

        assert_eq!(led.r, 51);
        assert_eq!(led.g, 102);
        assert_eq!(led.b, 204);
    }
}
