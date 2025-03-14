#[cfg(feature = "eval-track")]
use core::cell::RefCell;
use std::ops::{Add, AddAssign, Mul, MulAssign};

use arrayvec::ArrayVec;

use super::Eval;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Tracker<'a> {
    pub value: i16,
    #[cfg(feature = "eval-track")]
    pub track: ArrayVec<(&'a RefCell<TrackerNode>, f64), 256>,
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

impl Tracker<'_> {
    pub fn finalize() {
        todo!();
    }
}

impl<'a> From<&'a Weight> for Tracker<'a> {
    fn from(value: &'a Weight) -> Self {
        #[cfg(feature = "eval-track")]
        let mut track = ArrayVec::new_const();
        #[cfg(feature = "eval-track")]
        track.push((&value.track, 1.0));

        Self {
            value: value.value,
            #[cfg(feature = "eval-track")]
            track,
        }
    }
}

impl<'a> AddAssign<Tracker<'a>> for Tracker<'a> {
    fn add_assign(&mut self, rhs: Tracker<'a>) {
        self.value += rhs.value;
        #[cfg(feature = "eval-track")]
        self.track.extend(rhs.track);
    }
}

impl<'a> MulAssign<i16> for Tracker<'a> {
    fn mul_assign(&mut self, rhs: i16) {
        self.value *= rhs;

        for i in self.track.iter_mut() {
            i.1 *= rhs as f64;
        }
    }
}
