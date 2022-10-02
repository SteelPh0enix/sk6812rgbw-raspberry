use std::{error::Error, thread, time::Duration};

use palette::{FromColor, Gradient, Hsl, Hsv, LinSrgb, Srgb};
use sk6812_rpi::led::Led;

mod common;

#[test]
fn test_strip_single_color_fill() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();

    strip.fill(Led {
        r: 150,
        g: 0,
        b: 100,
        w: 0,
    });
    strip.update()?;

    Ok(())
}

#[test]
#[ignore]
fn show_all_colors() -> Result<(), Box<dyn Error>> {
    const COLOR_DELAY: Duration = Duration::from_millis(500);
    let test_colors: Vec<Led> = [
        [100, 0, 0].into(),
        [0, 100, 0].into(),
        [0, 0, 100].into(),
        [100, 100, 0].into(),
        [100, 0, 100].into(),
        [0, 100, 100].into(),
        [100, 100, 100].into(),
        [0, 0, 0, 100].into(),
        Default::default(),
    ]
    .to_vec();

    let mut strip = common::make_strip();

    test_colors.iter().for_each(|led| {
        strip.fill(*led);
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);
    });

    Ok(())
}

#[test]
fn test_strip_clearing() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();

    strip.clear();
    strip.update()?;

    Ok(())
}

#[test]
fn test_strip_gradient() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();
    let colors: Vec<LinSrgb> = (0..=360)
        .map(|i| Srgb::from_color(Hsv::new(i as f32, 1.0, 1.0)).into_linear())
        .collect();

    let gradient = Gradient::new(colors);

    strip.set_gradient(gradient);
    strip.update()?;

    Ok(())
}

#[test]
fn test_strip_gradient_shifting() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();
    let colors: Vec<LinSrgb> = (0..=360)
        .map(|i| Srgb::from_color(Hsv::new(i as f32, 1.0, 1.0)).into_linear())
        .collect();

    let shift_delay = Duration::from_millis(50);

    strip.set_gradient(Gradient::new(colors));
    strip.update()?;

    (0..strip.leds.len()).for_each(|_| {
        strip <<= 1;
        strip.update().unwrap();
        thread::sleep(shift_delay);
    });

    Ok(())
}

#[test]
fn test_direct_led_access() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();

    // Direct access to Led fields
    strip.leds[0].r = 100;
    strip.leds[1].g = 150;
    strip.leds[2].b = 200;

    // Conversion from arrays (RGB and RGBW, depending on the amount of items)
    strip.leds[3] = [100, 150, 200].into();
    strip.leds[4] = [100, 150, 200, 50].into();

    // Alternative way - use functions. Works exactly the same.
    strip.leds[5] = Led::from_rgb(100, 150, 200);
    strip.leds[6] = Led::from_rgbw(100, 150, 200, 50);

    // Conversion from `palette` types
    // Only f32 color types are currently supported
    strip.leds[7] = Srgb::new(0.2, 0.4, 0.6).into();
    strip.leds[8] = Hsv::new(0.5, 1.0, 1.0).into();
    strip.leds[9] = Hsl::new(0.85, 0.8, 0.5).into();

    Ok(())
}

#[test]
fn test_led_manipulation() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();

    strip.fill(Led::from_rgb(200, 100, 50));

    strip.leds[0] /= 2;
    strip.leds[1] *= 1.2;
    strip.leds[2] += 50;

    assert_eq!(strip.leds[0], Led::from_rgb(100, 50, 25));
    assert_eq!(strip.leds[1], Led::from_rgb(240, 120, 60));
    assert_eq!(strip.leds[2], Led::from_rgbw(250, 150, 100, 50));

    strip.update()?;

    Ok(())
}

#[test]
fn test_led_iter_manipulation() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();

    strip
        .leds
        .iter_mut()
        .enumerate()
        .for_each(|(index, led)| led.w = index as u8);

    strip.update()?;

    Ok(())
}
