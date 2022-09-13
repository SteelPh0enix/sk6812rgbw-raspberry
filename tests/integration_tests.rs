use std::{time::Duration, error::Error, thread};

use sk6812::led::Led;

mod common;

#[test]
fn test_strip_single_color_fill() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();

    strip.set_color(&Led {
        r: 250,
        g: 0,
        b: 200,
        w: 10,
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
        strip.set_color(led);
        strip.update().unwrap();
        thread::sleep(COLOR_DELAY);
    });

    Ok(())
}

#[test]
fn test_strip_clearing() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();

    strip.clear();
    strip.update().unwrap();

    Ok(())
}
