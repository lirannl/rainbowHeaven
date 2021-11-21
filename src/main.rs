use lazy_static::lazy_static;
use regex::Regex;
use rppal::gpio::Gpio;
use std::error::Error;
use std::io::{self};
use std::process::{self};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

static GPIO_PIN_NUM: u8 = 14;
lazy_static! {
    static ref BIN_STR_REGEX: Regex = Regex::new("^[01]+$").unwrap();
}

fn main() {
    let (tx, rx) = channel::<Vec<bool>>();
    spawn(move || match signalling_thread(rx) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error activating pin {}: {}", GPIO_PIN_NUM, error);
            process::exit(0);
        }
    });
    stdin_loop(&tx);
}

fn signalling_thread(rx: Receiver<Vec<bool>>) -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_PIN_NUM)?.into_output();
    for seq in rx.into_iter() {
        flash_pattern(seq, &mut pin);
        // Wait 1 second between signals
        sleep(Duration::from_secs(1));
    }
    Ok(())
}

fn stdin_loop(tx: &Sender<Vec<bool>>) {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                let sliced = &input[..n - 1];
                match sliced {
                    "exit" => {
                        process::exit(0);
                    }
                    // Capture binary strings and convert them to on-off states
                    binary_string if BIN_STR_REGEX.is_match(binary_string) => {
                        match tx.send(binary_string.chars().map(|c| c == '1').collect()) {
                            Ok(_) => {}
                            Err(error) => {
                                eprintln!("Failed to send signal {}: {}", binary_string, error)
                            }
                        }
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
