use core::cell::Cell;
use core::marker::PhantomData;
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[cfg(feature = "eval-track")]
use arrayvec::ArrayVec;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Tracker<'a> {
    value: i16,
    #[cfg(feature = "eval-track")]
    pub track: ArrayVec<(&'a WeightCell, f64), 256>,
    _phantom: PhantomData<&'a ()>,
}

pub type WeightCell = Cell<Weight>;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Weight {
    pub value: i16,
    #[cfg(feature = "eval-track")]
    derivative: f64,
    #[cfg(feature = "eval-track")]
    frequency: usize,
}

impl Weight {
    pub const fn new(value: i16) -> Self {
        Self {
            value,
            #[cfg(feature = "eval-track")]
            derivative: 0.0,
            #[cfg(feature = "eval-track")]
            frequency: 0,
        }
    }

    #[cfg(feature = "eval-track")]
    pub const fn derivative(&self) -> f64 { self.derivative }
    #[cfg(feature = "eval-track")]
    pub const fn frequency(&self) -> usize { self.frequency }
}

impl Tracker<'_> {
    pub fn value(&self) -> i16 { self.value }

    #[cfg(feature = "eval-track")]
    pub fn backprop(&self) {}
}

impl<'a> From<&'a WeightCell> for Tracker<'a> {
    fn from(value: &'a WeightCell) -> Self {
        #[cfg(feature = "eval-track")]
        let mut track = ArrayVec::new_const();
        #[cfg(feature = "eval-track")]
        track.push((value, 1.0));

        Self {
            value: value.get().value,
            #[cfg(feature = "eval-track")]
            track,
            _phantom: PhantomData,
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

impl<'a> SubAssign<Tracker<'a>> for Tracker<'a> {
    fn sub_assign(&mut self, rhs: Tracker<'a>) {
        self.value += rhs.value;
        #[cfg(feature = "eval-track")]
        self.track.extend(rhs.track.into_iter().map(|i| (i.0, -i.1)));
    }
}

impl<'a> MulAssign<i16> for Tracker<'a> {
    fn mul_assign(&mut self, rhs: i16) {
        self.value *= rhs;

        #[cfg(feature = "eval-track")]
        for i in self.track.iter_mut() {
            i.1 *= rhs as f64;
        }
    }
}

impl<'a> DivAssign<i16> for Tracker<'a> {
    fn div_assign(&mut self, rhs: i16) {
        self.value /= rhs;

        #[cfg(feature = "eval-track")]
        for i in self.track.iter_mut() {
            i.1 /= rhs as f64;
        }
    }
}
