mod config;
mod util;

use lineup::write;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let cfg = config::Config::new();
    let out_fmt = cfg.out_format();
    let ostream = cfg.ostream();
    let iseparator = cfg.in_separator();
    let mut istream = cfg.istream();
    let mut istream_buf = "".to_string();
    istream.read_to_string(&mut istream_buf)?;
    let istream = util::to_iter(istream_buf.as_str(), iseparator);
    write(istream, ostream, out_fmt)?;
    Ok(())
}
