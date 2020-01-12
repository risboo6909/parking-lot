mod internals;

use crate::parking_lot::internals::{MyResult, ParkLotIntern, Slot};
use failure::{bail, Error};
use std::string::ToString;

#[derive(Copy, Clone, Debug)]
enum Action<'a> {
    Create(usize),
    Park(&'a str, &'a str),
    Leave(usize),
    Status,
    SlotNumbersForColor(&'a str),
    SlotNumbersForRegNo(&'a str),
    RegNumbersForColor(&'a str),
}

pub(crate) fn stringify(to_print: Result<Vec<String>, Error>) -> String {
    match to_print {
        Ok(items) => items.join("\n"),
        Err(err) => format!("{}", err),
    }
}

pub(crate) struct ParkingLot {
    internal: internals::ParkLotIntern,
}

impl ParkingLot {
    pub(crate) fn new() -> Self {
        ParkingLot {
            internal: ParkLotIntern::new(),
        }
    }

    fn process(&mut self, action: Action) -> MyResult {
        match action {
            Action::Create(capacity) => self.internal.alloc(capacity),
            Action::Park(reg_num, color) => self.internal.park(reg_num, color),
            Action::Leave(slot_no) => self.internal.leave(slot_no),
            Action::Status => self.internal.status(),

            Action::RegNumbersForColor(color) => {
                self.internal.query(&|_, slot| -> Option<String> {
                    match slot {
                        Slot::Occupied(car) => {
                            if car.color_match(color) {
                                Some(car.reg_num())
                            } else {
                                None
                            }
                        }
                        Slot::Empty => None,
                    }
                })
            }

            Action::SlotNumbersForColor(color) => {
                self.internal.query(&|idx, slot| -> Option<String> {
                    match slot {
                        Slot::Occupied(car) => {
                            if car.color_match(color) {
                                Some(idx.to_string())
                            } else {
                                None
                            }
                        }
                        Slot::Empty => None,
                    }
                })
            }

            Action::SlotNumbersForRegNo(reg_num) => {
                self.internal.query(&|idx, slot| -> Option<String> {
                    match slot {
                        Slot::Occupied(car) => {
                            if car.reg_num_match(reg_num) {
                                Some(idx.to_string())
                            } else {
                                None
                            }
                        }
                        Slot::Empty => None,
                    }
                })
            }
        }
    }

    /// Receive a command as a string, parse and execute it
    ///
    /// # Example
    /// ```
    /// let mut parking = ParkingLot::new();
    /// parking.repl("create_parking_lot 6");
    /// ```
    ///
    pub(crate) fn repl(&mut self, command: &str) -> MyResult {
        let tokens: Vec<&str> = command.split_ascii_whitespace().collect();

        let action = match &tokens[..] {
            ["create_parking_lot", capacity] => Action::Create(capacity.parse::<usize>()?),
            ["park", reg_num, color] => Action::Park(reg_num, color),
            ["leave", slot_no] => Action::Leave(slot_no.parse::<usize>()?),
            ["status"] => Action::Status,
            ["registration_numbers_for_cars_with_colour", color] => {
                Action::RegNumbersForColor(color)
            }
            ["slot_numbers_for_cars_with_colour", color] => Action::SlotNumbersForColor(color),
            ["slot_number_for_registration_number", reg_num] => {
                Action::SlotNumbersForRegNo(reg_num)
            }

            _ => bail!("Can't parse command: {}", command),
        };

        self.process(action)
    }
}

#[cfg(test)]
mod tests {

    use super::{stringify, ParkingLot};
    use failure::Error;

    #[test]
    fn test_repl_ok() -> Result<(), Error> {
        let mut parking = ParkingLot::new();
        assert_eq!(
            &parking.repl("create_parking_lot 6")?.pop().unwrap(),
            "Created a parking lot with 6 slots"
        );
        Ok(())
    }

    #[test]
    fn test_repl_unknown_command() -> Result<(), Error> {
        let mut parking = ParkingLot::new();
        assert_eq!(
            format!("{}", parking.repl("this is sparta").err().unwrap()),
            "Can't parse command: this is sparta"
        );
        Ok(())
    }

    #[test]
    fn test_repl_input_from_file() -> Result<(), Error> {
        let input_raw = include_str!("test_input");
        let expected_raw = include_str!("test_exp");

        let mut parking = ParkingLot::new();
        for (command, exp) in input_raw.lines().zip(expected_raw.lines()) {
            let mut resp = stringify(parking.repl(command));
            resp.retain(|c| c != '\n' && c != ' ');
            let mut exp_norm = String::from(exp);
            exp_norm.retain(|c| c != '\n' && c != ' ');

            assert_eq!(resp, exp_norm);
        }

        Ok(())
    }
}
