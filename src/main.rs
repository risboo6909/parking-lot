use std::{self, env, fs};

mod parking_lot;

enum Mode {
    File,
    Interactive,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = if args.len() == 1 {
        Mode::Interactive
    } else {
        Mode::File
    };

    let mut parking = parking_lot::ParkingLot::new();

    match mode {
        Mode::Interactive => loop {
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).unwrap();
            println!("{}", parking_lot::stringify(parking.repl(&command)));
        },
        Mode::File => {
            let contents =
                fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
            for command in contents.lines() {
                println!("\ncommand: '{}'", command);
                println!("{}", parking_lot::stringify(parking.repl(&command)));
            }
        }
    }
}
