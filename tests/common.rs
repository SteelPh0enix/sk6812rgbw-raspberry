use sk6812_rpi::strip::{Bus, Strip};

pub fn make_strip() -> Strip {
    Strip::new(Bus::Spi0, 144).unwrap()
}
