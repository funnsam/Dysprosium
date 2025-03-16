use core::cell::Cell;
use core::marker::PhantomData;
use std::{fmt, ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

#[cfg(feature = "eval-track")]
use arrayvec::ArrayVec;

#[derive(Default, Clone, PartialEq, PartialOrd)]
pub struct Tracker<'a, T> {
    value: T,
    #[cfg(feature = "eval-track")]
    track: ArrayVec<(&'a WeightCell, f64), 256>,
    _phantom: PhantomData<&'a ()>,
}

#[derive(Default, Clone, PartialEq, PartialOrd)]
pub struct WeightCell(Cell<Weight>);

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Weight {
    pub value: i16,
    #[cfg(feature = "eval-track")]
    derivative: f64,
}

impl<T: Copy> Tracker<'_, T> {
    pub const fn value(&self) -> T { self.value }
}

pub fn sigmoid(x: f64, k: f64) -> f64 {
    ((-k * x).exp() + 1.0).recip()
}

impl Tracker<'_, f64> {
    #[cfg(feature = "eval-track")]
    pub fn backprop(self, r: f64, k: f64) -> f64 {
        let y_hat = sigmoid(self.value(), k);

        for (t, m) in self.track.iter() {
            if *m == 0.0 { continue };

            let term = t.get();
            t.set(Weight {
                derivative: term.derivative + 2.0 * k * m * y_hat * (y_hat - r) * (1.0 - y_hat),
                ..term
            });
        }

        (r - y_hat).powi(2)
    }
}

impl<T: fmt::Debug> fmt::Debug for Tracker<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tracker")
            .field("value", &self.value)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for WeightCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Weight::new({}).into()", self.0.get().value)
    }
}

impl From<Weight> for WeightCell {
    fn from(value: Weight) -> Self {
        Self(value.into())
    }
}

impl Deref for WeightCell {
    type Target = Cell<Weight>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Weight");
        s.field("w", &self.value);

        #[cfg(feature = "eval-track")]
        {
            s.field("∂C", &self.derivative());
        }

        s.finish()
    }
}

impl Weight {
    pub const fn new(value: i16) -> Self {
        Self {
            value,
            #[cfg(feature = "eval-track")]
            derivative: 0.0,
        }
    }

    #[cfg(feature = "eval-track")]
    pub const fn derivative(&self) -> f64 { self.derivative }

    pub const fn reset_meta(&mut self) {
        #[cfg(feature = "eval-track")]
        {
            self.derivative = 0.0;
        }
    }
}

impl<'a> From<&'a WeightCell> for Tracker<'a, i16> {
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

impl<'a, T> Tracker<'a, T> {
    // NOTE: does not impl `From` due to the inability of specialization
    pub fn from<F: Into<T>>(value: Tracker<'a, F>) -> Self {
        Self {
            value: value.value.into(),
            #[cfg(feature = "eval-track")]
            track: value.track,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: AddAssign<T> + Copy> AddAssign<Tracker<'a, T>> for Tracker<'a, T> {
    #[cfg_attr(not(feature = "eval-track"), inline)]
    fn add_assign(&mut self, rhs: Tracker<'a, T>) {
        self.value += rhs.value;
        #[cfg(feature = "eval-track")]
        self.track.extend(rhs.track);
    }
}

impl<'a> AddAssign<(&'a WeightCell, i16)> for Tracker<'a, i16> {
    #[cfg_attr(not(feature = "eval-track"), inline)]
    fn add_assign(&mut self, rhs: (&'a WeightCell, i16)) {
        self.value += rhs.0.get().value * rhs.1;
        #[cfg(feature = "eval-track")]
        self.track.push((rhs.0, rhs.1 as f64));
    }
}

impl<'a> AddAssign<&'a WeightCell> for Tracker<'a, i16> {
    #[inline]
    fn add_assign(&mut self, rhs: &'a WeightCell) {
        *self += (rhs, 1);
    }
}

impl<'a, T: SubAssign<T> + Copy> SubAssign<Tracker<'a, T>> for Tracker<'a, T> {
    #[cfg_attr(not(feature = "eval-track"), inline)]
    fn sub_assign(&mut self, rhs: Tracker<'a, T>) {
        self.value -= rhs.value;
        #[cfg(feature = "eval-track")]
        self.track.extend(rhs.track.into_iter().map(|i| (i.0, -i.1)));
    }
}

impl<'a> SubAssign<(&'a WeightCell, i16)> for Tracker<'a, i16> {
    #[cfg_attr(not(feature = "eval-track"), inline)]
    fn sub_assign(&mut self, rhs: (&'a WeightCell, i16)) {
        self.value -= rhs.0.get().value * rhs.1;
        #[cfg(feature = "eval-track")]
        self.track.push((rhs.0, -(rhs.1 as f64)));
    }
}

impl<'a> SubAssign<&'a WeightCell> for Tracker<'a, i16> {
    #[inline]
    fn sub_assign(&mut self, rhs: &'a WeightCell) {
        *self -= (rhs, 1);
    }
}

impl<'a, T: MulAssign<T> + Into<f64> + Copy> MulAssign<T> for Tracker<'a, T> {
    #[cfg_attr(not(feature = "eval-track"), inline)]
    fn mul_assign(&mut self, rhs: T) {
        self.value *= rhs;

        #[cfg(feature = "eval-track")]
        for i in self.track.iter_mut() {
            i.1 *= rhs.into();
        }
    }
}

impl<'a, T: DivAssign<T> + Into<f64> + Copy> DivAssign<T> for Tracker<'a, T> {
    #[cfg_attr(not(feature = "eval-track"), inline)]
    fn div_assign(&mut self, rhs: T) {
        self.value /= rhs;

        #[cfg(feature = "eval-track")]
        for i in self.track.iter_mut() {
            i.1 /= rhs.into();
        }
    }
}

impl<'a, T> Add<Tracker<'a, T>> for Tracker<'a, T> where Self: AddAssign<Self> {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, T> Sub<Tracker<'a, T>> for Tracker<'a, T> where Self: SubAssign<Self> {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<'a, T> Mul<T> for Tracker<'a, T> where Self: MulAssign<T> {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<'a, T> Div<T> for Tracker<'a, T> where Self: DivAssign<T> {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}
