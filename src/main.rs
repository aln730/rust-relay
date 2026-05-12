use gpio_cdev::{Chip, LineRequestFlags};
use std::io;

fn main() {
    // BCM GPIO pins for relays
    let pins = [4, 17, 18, 27, 22, 23, 24, 25];

    let mut chip = Chip::new("/dev/gpiochip0").unwrap();

    // Get relay handles
    let mut relays = Vec::new();

    for pin in pins {
        let relay = chip
            .get_line(pin)
            .unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "relay")
            .unwrap();

        relays.push(relay);
    }

    loop {
        println!("Enter relay number (1-8), 0 for OFF:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let num: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        // Turn all relays OFF first
        for relay in &relays {
            relay.set_value(1).unwrap();
        }

        // Turn selected relay ON
        if num >= 1 && num <= 8 {
            relays[num - 1].set_value(0).unwrap();
            println!("Relay {} ON", num);
        } else {
            println!("All relays OFF");
        }
    }
}