use std::{io::{self, Write}, ops::Range};

pub static PARAMS: &[Param] = &[
    Param {
        name: "rfp_margin_coeff",
        typ: Type::Int,
        default: 150.0,
        bound: 50.0..250.0,
    }
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
