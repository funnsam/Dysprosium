/// Evaluation score in centipawns. +ve is side to move better and -ve is worse
/// ```text
///    ┌┬┬─ mate in n              ┌┬┬─ mate in !n
/// 10_000…b                    01_111…b
/// -32767                      32767
/// #-0                         #0
/// ←──────|──────|──────|──────→
///      min cp   0   max cp
///      -16383        16383
/// ```
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Eval(pub i16);

impl Eval {
    pub const MAX: Self = Self(i16::MAX);
    pub const MIN: Self = Self(-Self::MAX.0);
    pub const M0: Self = Self(Self::MAX.0);

    #[inline]
    pub fn incr_mate(self) -> Self {
        match self.0 as u16 >> 14 {
            1 => Self(self.0 - 1),
            2 => Self(self.0 + 1),
            _ => self,
        }
    }

    #[inline]
    pub fn is_mate(self) -> bool {
        matches!(self.0 as u16 >> 14, 1 | 2)
    }

    #[inline]
    pub fn is_positive_mate(self) -> bool {
        self.0 as u16 >> 14 == 1
    }
}

impl core::ops::Add<i16> for Eval {
    type Output = Self;

    #[inline]
    fn add(self, rhs: i16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl core::ops::Sub<i16> for Eval {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: i16) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl core::ops::Sub<Eval> for i16 {
    type Output = Eval;

    #[inline]
    fn sub(self, rhs: Eval) -> Self::Output {
        Eval(self - rhs.0)
    }
}

impl core::ops::Neg for Eval {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        if self.is_mate() {
            Self(!self.0)
        } else {
            Self(-self.0)
        }
    }
}

impl core::fmt::Display for Eval {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            match self.0 as u16 >> 14 {
                1 => write!(f, "mate {}", !self.0 & 0x3fff),
                2 => write!(f, "mate -{}", self.0 & 0x3fff),
                _ => write!(f, "cp {}", self.0),
            }
        } else {
            match self.0 as u16 >> 14 {
                1 => write!(f, "#{}", ((!self.0 & 0x3fff) + 1) / 2),
                2 => write!(f, "#-{}", ((self.0 & 0x3fff) + 1) / 2),
                _ => write!(f, "{}cp", self.0),
            }
        }
    }
}

#[test]
#[cfg(test)]
fn test_eval() {
    let m0 = Eval::M0;
    let m1 = m0.incr_mate();
    let m_0 = -m0;
    let m_1 = m_0.incr_mate();

    assert_eq!(m0.0, 0x7fff);
    assert_eq!(m0.to_string(), "#0");
    assert_eq!(m1.0, 0x7ffe);
    assert_eq!(m1.to_string(), "#1");
    assert_eq!(m_0.0 as u16, 0x8000);
    assert_eq!(m_0.to_string(), "#-0");
    assert_eq!(m_1.0 as u16, 0x8001);
    assert_eq!(m_1.to_string(), "#-1");

    let m0 = -m_0;
    assert_eq!(m0.0, 0x7fff);

    assert_eq!(--m0, m0);
    assert_eq!(--m1, m1);
    assert_eq!(--m_0, m_0);
    assert_eq!(--m_1, m_1);
    assert_eq!(-m0, m_0);
    assert_eq!(-m1, m_1);
    assert_eq!(-m_0, m0);
    assert_eq!(-m_1, m1);
}
