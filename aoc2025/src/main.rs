#[macro_use]
extern crate maplit;

mod helpers;
mod basic_parsing;
mod vec2;
mod matrix;
mod maze;
mod find_union;

mod playground;

mod dec1;
mod dec2;
mod dec3;

fn main() {
    //playground::play();
    dec3::dec3_2();
}
