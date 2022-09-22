# sk6812_rpi

Easy-to-use SK6812RGBW library for RaspberryPi.

## Features

* Easy strip creation - just set the SPI bus you wanna use, and set the number of LEDs.
* Easy access to LED colors - LEDs are stored as a `Vec` inside `Strip` struct, so you can access them however you want.
* [`palette`](https://crates.io/crates/palette) integration - [`palette`](https://crates.io/crates/palette) is a library dedicated for color manipulation, `LED` struct have implemented `From`/`Into` traits that allow for easy conversion from/to [`palette`](https://crates.io/crates/palette) SRGB type. In other words - you can easely create, modify or blend LED colors in any way this library allows.
* Gradient support - since [`palette`](https://crates.io/crates/palette) has [gradient](https://docs.rs/palette/0.6.1/palette/gradient/struct.Gradient.html) support, you can just pass one to `Strip` and it'll be automatically pushed onto LEDs.
* Color position shifting support - you can shift the positions of LED colors to make smooth animations using `<<=` and `>>=` operators on `Strip` structure instance.

## Compatibility

RaspberryPi compatibility is enforced by [`rppal`](https://crates.io/crates/rppal) library. In other words - it should work with any RaspberryPi with GPIO header. See [`rppal`](https://crates.io/crates/rppal) readme for details.

Should work with any SK6812RGBW strip, or similar (WS2812-like) assuming that it uses GRBW color format. Modifying it for different formats should be pretty straighforward, but i currenlty have no time or need to extend this library like that, so feel free to fork it and modify it yourself. It should be fairly simple, see below for more details.

## Installation

Just add it to your `Cargo.toml`

```toml
[dependencies]
sk6812_rpi = "1.0"
```

## Usage and examples

### Creating a strip

```rust
use sk6812_rpi::strip::{Bus, Strip};

let mut strip = Strip::new(Bus::Spi0, 144).unwrap()
```

### Setting the strip to a specific RGB(W) color

```rust
use sk6812_rpi::strip::{Bus, Strip};
use sk6812_rpi::led::Led;

let mut strip = Strip::new(Bus::Spi0, 144).unwrap()

strip.fill(Led {
    r: 200,
    g: 0,
    b: 150,
    w: 0,
});

strip.update().unwrap();
```

### Setting the strip to a gradient color

```rust
use sk6812_rpi::strip::{Bus, Strip}
use palette::{FromColor, Gradient, Hsv, LinSrgb, Srgb};

let mut strip = Strip::new(Bus::Spi0, 144).unwrap();

let colors: Vec<LinSrgb> = (0..=360)
    .map(|i| Srgb::from_color(Hsv::new(i as f32, 1.0, 0.8)).into_linear())
    .collect();

let gradient = Gradient::new(colors);

strip.set_gradient(gradient);
strip.update().unwrap();
```

For more examples and extended usage, look into [tests](./tests) and [src](./src) directory. There are tests for every module, presenting how to use most of the functions available.

## Common issues

### SPI message is too long, program crashes on `Strip::update` call

This is caused by default Raspbian SPI buffer size of 4096 bytes. To change it, edit `/boot/cmdline.txt` file and `spidev.bufsiz=65535` to the command line.

In my case, it looks like this:

```sh
console=tty1 root=PARTUUID=09632905-02 rootfstype=ext4 fsck.repair=yes spidev.bufsiz=65535 rootwait
```

Yours may look differently, but the important part is to add the `spidev.bufsiz=65535` there. **Reboot your Raspberry** and it should work.

## Modifying the library to use LEDs with different color format

If you want to use this library with LEDs that don't use GRBW format, you have to modify `Led::to_raw_led_bytes` function (it's in [led.rs](./src/led.rs) file). By default, it looks like this:

```rust
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
```

As you can see, it creates an array of colors and gets the bits using `view_bits` from [`bitvec`](https://crates.io/crates/bitvec) library. To change the order of colors, just change the order of elements in the array. If you don't have a white channel, remove it. That's it. Rest of the code is generic and will adapt to the changes automatically.

If you'll make a generic version of `Led` supporting multiple color , please make a pull request and i'll gladly merge it. Should be fairly simple, but i currently have no time nor need to do so.
