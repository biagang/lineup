mod config;

use lineup::{read, write};
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let cfg = config::Config::new();

    let mut istream = cfg.istream();
    let mut buf = "".to_string();
    istream.read_to_string(&mut buf)?;
    let item_reader = read(buf.as_str(), cfg.in_fmt().clone());
    write(item_reader, cfg.ostream(), cfg.out_format())?;
    Ok(())
}
