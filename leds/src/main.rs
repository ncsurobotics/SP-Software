use smart_leds::{brightness, RGB8, SmartLedsWrite, colors::{RED, GREEN, BLUE, WHITE}};
use ws2812_spi:: Ws2812 as ws2812;

fn main() {
    let leds = ws2812:: new (spi);
    const NUM_LEDS: usize = 8;

    let mut colorCollection: Vec<[smart_leds::RGB8; NUM_LEDS]> = Vec::new();
    let white : [RGB8; NUM_LEDS] = [WHITE; NUM_LEDS];

    colorCollection.push(white);
}
