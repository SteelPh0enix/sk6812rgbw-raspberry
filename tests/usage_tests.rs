use std::{error::Error, thread, time::Duration};

use sk6812::led::Led;

mod common;

#[test]
fn test_strip_single_color_fill() -> Result<(), Box<dyn Error>> {
    let mut strip = common::make_strip();

    strip.fill(Led {
        r: 200,
        g: 0,
        b: 150,
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
    strip.update().unwrap();

    Ok(())
}
