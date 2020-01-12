use failure::{bail, format_err, Error};

pub(crate) type MyResult = Result<Vec<String>, Error>;

#[derive(Clone)]
pub(crate) struct Car {
    reg_num: String,
    color: String,
}

impl Car {
    pub(crate) fn reg_num_match(&self, reg_num: &str) -> bool {
        self.reg_num.to_lowercase() == reg_num.to_lowercase()
    }

    pub(crate) fn color_match(&self, color: &str) -> bool {
        self.color.to_lowercase() == color.to_lowercase()
    }

    pub(crate) fn reg_num(&self) -> String {
        self.reg_num.to_owned()
    }
}

#[derive(Clone)]
pub(crate) enum Slot {
    Occupied(Car),
    Empty,
}

pub(crate) struct ParkLotIntern {
    slots: Option<Vec<Slot>>,
}

impl ParkLotIntern {
    pub(crate) fn new() -> Self {
        ParkLotIntern { slots: None }
    }

    pub(crate) fn alloc(&mut self, n: usize) -> MyResult {
        if self.slots.is_some() {
            Err(format_err!("Parking slots already allocated"))
        } else if n == 0 {
            Err(format_err!("Parking lot size must be greater than 0"))
        } else {
            self.slots = Some(vec![Slot::Empty; n as usize]);
            Ok(vec![format!("Created a parking lot with {} slots", n)])
        }
    }

    fn check_allocated(&self) -> Result<(), Error> {
        if self.slots.is_none() {
            Err(format_err!("Parking slots are not allocated"))
        } else {
            Ok(())
        }
    }

    pub(crate) fn park(&mut self, reg_num: &str, color: &str) -> MyResult {
        self.check_allocated()?;

        let s_ref = self.slots.as_mut().unwrap();

        // find nearest empty slot
        let mut empty_idx = None;
        for (idx, slot) in s_ref.iter().enumerate() {
            match slot {
                Slot::Empty => {
                    empty_idx = Some(idx);
                    break;
                }
                Slot::Occupied(_) => {}
            }
        }

        // occupy slot if possible
        if let Some(idx) = empty_idx {
            s_ref[idx] = Slot::Occupied(Car {
                reg_num: reg_num.to_owned(),
                color: color.to_owned(),
            });
            Ok(vec![format!("Allocated slot number: {}", idx + 1)])
        } else {
            Err(format_err!("Sorry, parking lot is full"))
        }
    }

    pub(crate) fn leave(&mut self, slot_no: usize) -> MyResult {
        self.check_allocated()?;

        let s_ref = self.slots.as_mut().unwrap();

        if slot_no > s_ref.len() {
            bail!(
                "Invalid parking place number, please choose a number in interval [1, {}]",
                s_ref.len()
            );
        }

        s_ref[slot_no - 1] = Slot::Empty;

        Ok(vec![format!("Slot number {} is free", slot_no)])
    }

    pub(crate) fn status(&self) -> MyResult {
        self.check_allocated()?;

        let mut res = vec![String::from("Slot No.   Registration No  Colour")];

        res.extend(
            self.slots
                .as_ref()
                .unwrap()
                .iter()
                .enumerate()
                .filter_map(|(idx, slot)| match slot {
                    Slot::Occupied(car) => {
                        Some(format!("{}     {}  {}", idx + 1, car.reg_num, car.color))
                    }
                    Slot::Empty => None,
                }),
        );

        Ok(res)
    }

    pub(crate) fn query(&self, f: &dyn Fn(usize, &Slot) -> Option<String>) -> MyResult {
        self.check_allocated()?;

        let res = self
            .slots
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(idx, slot)| f(idx + 1, slot))
            .collect::<Vec<String>>();

        if res.is_empty() {
            Err(format_err!("Not found"))
        } else {
            Ok(res)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::ParkLotIntern;
    use failure::Error;

    #[test]
    fn test_alloc() -> Result<(), Error> {
        let mut p = ParkLotIntern::new();
        assert!(p.alloc(7).is_ok());
        assert!(p.alloc(6).is_err());
        Ok(())
    }

    #[test]
    fn test_park_leave() -> Result<(), Error> {
        let mut p = ParkLotIntern::new();

        p.alloc(3)?;

        assert!(p.park("T800", "Red").is_ok());
        assert!(p.park("T1000", "BLACK").is_ok());
        assert!(p.park("HAL9000", "GrEeN").is_ok());

        assert!(p.park("Submarine", "Yellow").is_err());

        assert!(p.leave(2).is_ok());
        assert!(p.leave(3).is_ok());

        assert_eq!(
            p.park("Submarine", "Yellow")?.pop().unwrap(),
            "Allocated slot number: 2"
        );

        Ok(())
    }
}
