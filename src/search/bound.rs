use std::ops::{Neg, RangeInclusive};

use crate::Eval;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bound {
    pub alpha: Eval,
    pub beta: Eval,
}

impl From<RangeInclusive<Eval>> for Bound {
    fn from(value: RangeInclusive<Eval>) -> Self {
        Self::new(*value.start(), *value.end())
    }
}

impl Neg for Bound {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.beta, -self.alpha)
    }
}

impl Bound {
    pub const MIN_MAX: Self = Self::new(Eval::MIN, Eval::MAX);

    pub const fn new(alpha: Eval, beta: Eval) -> Self {
        Self { alpha, beta }
    }

    pub fn from_window(g: Eval, aw: i16, bw: i16) -> Self {
        let alpha = Eval(g.0.saturating_sub(aw));
        let beta = Eval(g.0.saturating_sub(bw));

        Self::new(alpha, beta)
    }
}
