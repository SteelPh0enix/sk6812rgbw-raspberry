use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bitvec::prelude::*;
use palette::{rgb::Rgb, FromColor, Hsl, Hsv, Srgb};

/// High bit (logical 1) representation for SPI
const BIT_HIGH: u8 = 0b11110000;
/// Low bit (logical 0) representation for SPI
const BIT_LOW: u8 = 0b11000000;

/// Structure representing a single RGBW LED
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            w: 0,
        }
    }

    pub fn from_rgbw(red: u8, green: u8, blue: u8, white: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            w: white,
        }
    }

    pub fn from_rgb_array(data: [u8; 3]) -> Self {
        data.into()
    }

    pub fn from_rgbw_array(data: [u8; 4]) -> Self {
        data.into()
    }

    pub fn into_rgbw_array(self) -> [u8; 4] {
        self.into()
    }

    pub fn into_rgb_array(self) -> [u8; 3] {
        self.into()
    }

    /// Converts the instance of this struct to SK6812-compatible byte array for SPI.
    /// Don't use in your own code, unless you know what you're doing.
    pub fn to_raw_led_bytes(&self) -> Vec<u8> {
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

impl Add for Led {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Led::from_rgbw_array([
            self.r.checked_add(rhs.r).or(Some(u8::MAX)).unwrap(),
            self.g.checked_add(rhs.g).or(Some(u8::MAX)).unwrap(),
            self.b.checked_add(rhs.b).or(Some(u8::MAX)).unwrap(),
            self.w.checked_add(rhs.w).or(Some(u8::MAX)).unwrap(),
        ])
    }
}

impl Sub for Led {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Led::from_rgbw_array([
            self.r.checked_sub(rhs.r).or(Some(0)).unwrap(),
            self.g.checked_sub(rhs.g).or(Some(0)).unwrap(),
            self.b.checked_sub(rhs.b).or(Some(0)).unwrap(),
            self.w.checked_sub(rhs.w).or(Some(0)).unwrap(),
        ])
    }
}

impl Mul for Led {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Led::from_rgbw_array([
            self.r.checked_mul(rhs.r).or(Some(u8::MAX)).unwrap(),
            self.g.checked_mul(rhs.g).or(Some(u8::MAX)).unwrap(),
            self.b.checked_mul(rhs.b).or(Some(u8::MAX)).unwrap(),
            self.w.checked_mul(rhs.w).or(Some(u8::MAX)).unwrap(),
        ])
    }
}

impl Div for Led {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Led::from_rgbw_array([
            self.r.checked_div(rhs.r).or(Some(0)).unwrap(),
            self.g.checked_div(rhs.g).or(Some(0)).unwrap(),
            self.b.checked_div(rhs.b).or(Some(0)).unwrap(),
            self.w.checked_div(rhs.w).or(Some(0)).unwrap(),
        ])
    }
}

impl Add<u8> for Led {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self + Led::from_rgbw(rhs, rhs, rhs, rhs)
    }
}

impl Sub<u8> for Led {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        self - Led::from_rgbw(rhs, rhs, rhs, rhs)
    }
}

impl Mul<u8> for Led {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        self * Led::from_rgbw(rhs, rhs, rhs, rhs)
    }
}

impl Div<u8> for Led {
    type Output = Self;

    fn div(self, rhs: u8) -> Self::Output {
        self / Led::from_rgbw(rhs, rhs, rhs, rhs)
    }
}

impl Add<f32> for Led {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let rhs_u8: u8 = (rhs * (u8::MAX as f32)) as u8;
        self + Led::from_rgbw(rhs_u8, rhs_u8, rhs_u8, rhs_u8)
    }
}

impl Sub<f32> for Led {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let rhs_u8: u8 = (rhs * (u8::MAX as f32)) as u8;
        self - Led::from_rgbw(rhs_u8, rhs_u8, rhs_u8, rhs_u8)
    }
}

impl Mul<f32> for Led {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Led {
            r: (self.r as f32 * rhs) as u8,
            g: (self.g as f32 * rhs) as u8,
            b: (self.b as f32 * rhs) as u8,
            w: (self.w as f32 * rhs) as u8,
        }
    }
}

impl Div<f32> for Led {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Led {
            r: (self.r as f32 / rhs) as u8,
            g: (self.g as f32 / rhs) as u8,
            b: (self.b as f32 / rhs) as u8,
            w: (self.w as f32 / rhs) as u8,
        }
    }
}

impl AddAssign for Led {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Led {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Led {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Led {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl AddAssign<u8> for Led {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl SubAssign<u8> for Led {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self - rhs;
    }
}

impl MulAssign<u8> for Led {
    fn mul_assign(&mut self, rhs: u8) {
        *self = *self * rhs;
    }
}

impl DivAssign<u8> for Led {
    fn div_assign(&mut self, rhs: u8) {
        *self = *self / rhs;
    }
}

impl AddAssign<f32> for Led {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl SubAssign<f32> for Led {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Led {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Led {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl From<Led> for [u8; 3] {
    fn from(led: Led) -> Self {
        [led.r, led.g, led.b]
    }
}

impl From<Led> for [u8; 4] {
    fn from(led: Led) -> Self {
        [led.r, led.g, led.b, led.w]
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

impl From<Led> for Rgb {
    fn from(led: Led) -> Self {
        Rgb::new(
            (led.r as f32) / (u8::MAX as f32),
            (led.g as f32) / (u8::MAX as f32),
            (led.b as f32) / (u8::MAX as f32),
        )
    }
}

impl From<Rgb> for Led {
    fn from(color: Rgb) -> Self {
        [
            (color.red * (u8::MAX as f32)) as u8,
            (color.green * (u8::MAX as f32)) as u8,
            (color.blue * (u8::MAX as f32)) as u8,
        ]
        .into()
    }
}

impl From<Led> for Hsv {
    fn from(led: Led) -> Self {
        let srgb_color: Srgb = led.into();
        Hsv::from_color(srgb_color)
    }
}

impl From<Hsv> for Led {
    fn from(color: Hsv) -> Self {
        Srgb::from_color(color).into_format().into()
    }
}

impl From<Led> for Hsl {
    fn from(led: Led) -> Self {
        let srgb_color: Srgb = led.into();
        Hsl::from_color(srgb_color)
    }
}

impl From<Hsl> for Led {
    fn from(color: Hsl) -> Self {
        Srgb::from_color(color).into_format().into()
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
        let led_sk_bytes = led.to_raw_led_bytes();

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

        assert_eq!(led.into_rgbw_array(), [10, 20, 30, 40]);
        assert_eq!(led.into_rgb_array(), [10, 20, 30]);
    }

    #[test]
    fn test_led_from_array_creation() {
        let led_rgb = Led::from_rgb_array([10, 20, 30]);
        let led_rgbw = Led::from_rgbw_array([10, 20, 30, 40]);

        assert_eq!(led_rgb.into_rgb_array(), [10, 20, 30]);
        assert_eq!(led_rgbw.into_rgbw_array(), [10, 20, 30, 40]);
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
        let pixel = Srgb::new(0.2, 0.4, 0.8);
        let led: Led = pixel.into();

        assert_eq!(led.r, 51);
        assert_eq!(led.g, 102);
        assert_eq!(led.b, 204);
    }

    #[test]
    fn test_led_add() {
        let led_a = Led::from_rgbw(10, 20, 30, 40);
        let led_b = Led::from_rgbw(10, 10, 10, 10);

        let led_added = led_a + led_b;
        assert_eq!(led_added, Led::from_rgbw(20, 30, 40, 50));

        let mut led_c = led_a;
        led_c += led_b;
        assert_eq!(led_c, Led::from_rgbw(20, 30, 40, 50));

        let mut led_d = led_a;
        led_d += 10;
        assert_eq!(led_d, Led::from_rgbw(20, 30, 40, 50));

        let mut led_e = led_a;
        led_e += 0.1;
        assert_eq!(led_e, Led::from_rgbw(35, 45, 55, 65));
    }

    #[test]
    fn test_led_sub() {
        let led_a = Led::from_rgbw(10, 20, 30, 40);
        let led_b = Led::from_rgbw(10, 10, 10, 10);

        let led_subbed = led_a - led_b;
        assert_eq!(led_subbed, Led::from_rgbw(0, 10, 20, 30));

        let mut led_c = led_a;
        led_c -= led_b;
        assert_eq!(led_c, Led::from_rgbw(0, 10, 20, 30));

        let mut led_d = led_a;
        led_d -= 10;
        assert_eq!(led_d, Led::from_rgbw(0, 10, 20, 30));

        let mut led_e = led_a;
        led_e -= 0.1;
        assert_eq!(led_e, Led::from_rgbw(0, 0, 5, 15));
    }

    #[test]
    fn test_led_mul() {
        let led_a = Led::from_rgbw(10, 20, 30, 40);
        let led_b = Led::from_rgbw(3, 2, 1, 2);

        let led_multiplied = led_a * led_b;
        assert_eq!(led_multiplied, Led::from_rgbw(30, 40, 30, 80));

        let mut led_c = led_a;
        led_c *= led_b;
        assert_eq!(led_c, Led::from_rgbw(30, 40, 30, 80));

        let mut led_d = led_a;
        led_d *= 2;
        assert_eq!(led_d, Led::from_rgbw(20, 40, 60, 80));

        let mut led_e = led_a;
        led_e *= 0.5;
        assert_eq!(led_e, Led::from_rgbw(5, 10, 15, 20));
    }

    #[test]
    fn test_led_div() {
        let led_a = Led::from_rgbw(10, 20, 30, 40);
        let led_b = Led::from_rgbw(2, 2, 1, 4);

        let led_divided = led_a / led_b;
        assert_eq!(led_divided, Led::from_rgbw(5, 10, 30, 10));

        let mut led_c = led_a;
        led_c /= led_b;
        assert_eq!(led_c, Led::from_rgbw(5, 10, 30, 10));

        let mut led_d = led_a;
        led_d /= 2;
        assert_eq!(led_d, Led::from_rgbw(5, 10, 15, 20));

        let mut led_e = led_a;
        led_e /= 0.5;
        assert_eq!(led_e, Led::from_rgbw(20, 40, 60, 80));
    }
}
