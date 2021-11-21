use regex::Regex;
use rppal::gpio::Gpio;
use std::error::Error;
use std::io::{self};
use std::process::{self};
use std::thread::sleep;
use std::time::Duration;

static GPIO_PIN_NUM: u8 = 14;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut pin = Gpio::new()?.get(GPIO_PIN_NUM)?.into_output();
    main_loop(&mut input, &mut pin);
    Ok(())
}

fn main_loop(input: &mut std::string::String, pin: &mut rppal::gpio::OutputPin) {
    loop {
        match io::stdin().read_line(input) {
            Ok(n) => {
                let sliced = &input[..n - 1];
                match sliced {
                    //
                    "on" => pin.set_high(),
                    "off" => pin.set_low(),
                    "exit" => {
                        pin.set_low();
                        process::exit(0);
                    }
                    //
                    binary_string if Regex::new("^[01]+$").unwrap().is_match(binary_string) => {
                        flash_pattern(binary_string.chars().map(|c| c == '1').collect(), pin);
                    }
                    _ => println!("Unknown command {}", sliced),
                }
            }
            Err(error) => println!("error: {}", error),
        }
        input.clear();
    }
}

fn flash_pattern(bin_str: Vec<bool>, pin: &mut rppal::gpio::OutputPin) {
    // Go through the bits in the vec
    for bit in bin_str {
        if bit {
            pin.set_high();
        } else {
            pin.set_low();
        }
        sleep(Duration::from_millis(250));
    }
    // Always finish with the pin low
    pin.set_low();
}
