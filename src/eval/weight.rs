#[cfg(feature = "eval-track")]
use core::cell::RefCell;
use std::ops::Mul;

use super::Eval;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Tracker<'a> {
    pub value: i16,
    #[cfg(feature = "eval-track")]
    pub mul: f64,
    #[cfg(feature = "eval-track")]
    pub track: &'a RefCell<TrackerNode>,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Weight {
    pub value: i16,
    #[cfg(feature = "eval-track")]
    pub track: RefCell<TrackerNode>,
}

#[cfg(feature = "eval-track")]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct TrackerNode {
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
    pub fn reset_tracker(&self) -> TrackerNode {
        core::mem::take(&mut *self.track.borrow_mut())
    }
}

impl Tracker<'_> {}

impl<'a> Mul<&'a Weight> for Eval {
    type Output = Tracker<'a>;

    fn mul(self, rhs: &'a Weight) -> Self::Output {
        // #[cfg(feature = "eval-track")] {
        //     let mut track = rhs.track.borrow_mut();
        //     track.derivative += val as f64;
        //     track.frequency += 1;
        // }

        Tracker {
            value: self.0 * rhs.value,
            mul: 1.0,
            track: &rhs.track,
        }
    }
}
