use std::{io::{self, Write}, ops::Range};

pub static PARAMS: &[Param] = &[
    Param {
        name: "asp_init_delta",
        typ: Type::Int,
        default: 13.0,
        bound: 1.0..100.0,
    },
    Param {
        name: "rfp_ubound",
        typ: Type::Int,
        default: 2.0,
        bound: 1.0..16.0,
    },
    Param {
        name: "rfp_margin_coeff",
        typ: Type::Int,
        default: 120.0,
        bound: 10.0..200.0,
    },
    Param {
        name: "fp_ubound",
        typ: Type::Int,
        default: 2.0,
        bound: 2.0..16.0,
    },
    Param {
        name: "fp_margin_coeff",
        typ: Type::Int,
        default: 150.0,
        bound: 10.0..200.0,
    },
    Param {
        name: "lmr_coeff",
        typ: Type::Float,
        default: 0.4,
        bound: 0.1..3.0,
    },
    Param {
        name: "lmr_const",
        typ: Type::Float,
        default: 2.78,
        bound: 0.1..5.0,
    },
    Param {
        name: "lmr_improv",
        typ: Type::Float,
        default: 1.0,
        bound: 0.1..3.0,
    },
    Param {
        name: "lmr_pv",
        typ: Type::Float,
        default: 1.0,
        bound: 0.1..3.0,
    },
    Param {
        name: "hist_bonus_coeff",
        typ: Type::Int,
        default: 300.0,
        bound: 10.0..650.0,
    },
    Param {
        name: "hist_bonus_const",
        typ: Type::Int,
        default: -250.0,
        bound: -650.0..-10.0,
    },
    Param {
        name: "dp_big_delta",
        typ: Type::Int,
        default: 1100.0,
        bound: 700.0..2000.0,
    },
    Param {
        name: "dp_delta",
        typ: Type::Int,
        default: 200.0,
        bound: 50.0..500.0,
    },
];

#[derive(Debug, Clone)]
pub struct Param {
    pub name: &'static str,
    pub typ: Type,
    pub default: f64,
    pub bound: Range<f64>,
}

impl Param {
    pub fn c_end(&self) -> f64 {
        let diff = self.bound.end - self.bound.start;

        (diff / 20.0)
            .abs()
            .max(if self.typ == Type::Int { 0.5 } else { 0.0 })
            .min(25.0)
    }

    pub fn write_spsa(&self, s: &mut String) {
        let Self { name, typ, default, bound: Range { start, end }} = self;

        *s += &format!(
            "{name}, {}, {default}, {start}, {end}, {}, 0.002\n",
            typ.spsa_type(),
            self.c_end(),
        );
    }

    pub fn write_uci_option(&self, s: &mut String) {
        let Self { name, typ, default, bound: Range { start, end }} = self;

        *s += &format!(
            "option name {name} type {} default {default} min {start} max {end}\n",
            typ.uci_type(),
        );
    }

    pub fn write_uci_match<W: Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(
            w,
            "\"{}\" => self.{} = value.unwrap().parse::<{}>().unwrap(),",
            self.name,
            self.name,
            self.typ.rust_type(),
        )
    }

    pub fn write_struct<W: Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "pub {}: {},", self.name, self.typ.rust_type())
    }

    pub fn write_default<W: Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "{}: {}_{},", self.name, self.default, self.typ.rust_type())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int, Float
}

impl Type {
    pub fn uci_type(&self) -> &'static str {
        match self {
            Self::Int => "spin",
            Self::Float => "string",
        }
    }

    pub fn spsa_type(&self) -> &'static str {
        match self {
            Self::Int => "int",
            Self::Float => "float",
        }
    }

    pub fn rust_type(&self) -> &'static str {
        match self {
            Self::Int => "i16",
            Self::Float => "f32",
        }
    }
}
