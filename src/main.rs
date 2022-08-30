mod component;
use component::bus::Bus;

fn main() {
    let mut bus = Bus::new();
    bus.write(0x7e, 0x0000, 0x6c);
    let data = bus.read(0x00, 0x0000);
    match data {
        Some(x) => println!("{}", x),
        None => println!("Failure")
    }
}