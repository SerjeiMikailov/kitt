use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};
use std::thread;
use std::time::Duration;

fn main() {
    loop{
    Keyboard::A.bind(|_| {
        println!("A pressed, sending B");
        Keyboard::B.click();
    });
    }
}
