use enigo::*;
use std::thread::sleep;
use std::time;

pub fn run() {
    println!("Initiated.\n");

    let mut enigo = Enigo::new();
    let done = false;
    let second = time::Duration::from_secs(5);

    while !done {
        sleep(second);
        enigo.key_down(Key::Layout('w'));
    }

}
