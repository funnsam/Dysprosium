#[cfg(feature = "eval-track")]
use core::cell::RefCell;
use std::ops::Mul;

use super::Eval;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Weight {
    pub value: i16,
    #[cfg(feature = "eval-track")]
    pub track: RefCell<Tracker>,
}

#[cfg(feature = "eval-track")]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Tracker {
    pub derivative: f64,
    pub frequency: usize,
}

impl Weight {
    pub fn new(value: i16) -> Self {
        Self {
            value,
            #[cfg(feature = "eval-track")]
            track: RefCell::default(),
        }
    }

    #[cfg(feature = "eval-track")]
    pub fn reset_tracker(&self) -> Tracker {
        core::mem::take(&mut *self.track.borrow_mut())
    }
}

impl Mul<&Weight> for Eval {
    type Output = Eval;

    fn mul(self, rhs: &Weight) -> Self::Output {
        let val = self.0 * rhs.value;

        #[cfg(feature = "eval-track")] {
            let mut track = rhs.track.borrow_mut();
            track.derivative += val as f64;
            track.frequency += 1;
        }

        Eval(val)
    }
}
