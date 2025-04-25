use std::io::{self, Write};

use param::PARAMS;

mod param;

pub fn write_search_params<W: Write>(out: &mut W) -> io::Result<()> {
    writeln!(out, "// auto-generated")?;

    writeln!(out, "pub struct SearchParams {{")?;
    for param in PARAMS.iter() {
        param.write_struct(out)?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "impl Default for SearchParams {{")?;
    writeln!(out, "fn default() -> Self {{")?;
    writeln!(out, "Self {{")?;
    for param in PARAMS.iter() {
        param.write_default(out)?;
    }
    writeln!(out, "}}")?;
    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "impl SearchParams {{")?;

    let mut uci_opt = String::new();
    let mut spsa = String::new();
    for param in PARAMS.iter() {
        param.write_uci_option(&mut uci_opt);
        param.write_spsa(&mut spsa);
    }
    writeln!(out, "pub const UCI_OPTIONS: &str = {uci_opt:?};")?;
    writeln!(out, "pub const SPSA: &str = {spsa:?};")?;

    writeln!(out, "pub fn exec_setoption(&mut self, name: &str, value: Option<&str>) {{")?;
    writeln!(out, "match name {{")?;
    for param in PARAMS.iter() {
        param.write_uci_match(out)?;
    }
    writeln!(out, "_ => {{}},")?;
    writeln!(out, "}}")?;
    writeln!(out, "}}")?; // fn exec_setoption

    writeln!(out, "}}")?; // impl SearchParams

    Ok(())
}
