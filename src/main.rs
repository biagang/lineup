mod config;
use lineup::write;

fn main() {
    let cfg = config::Config::new();
    let in_sep = cfg.in_separator();
    let out_fmt = cfg.out_format();
    let ostream = cfg.ostream();
    let input = cfg.input();
    let istream = input.split(in_sep.as_str());
    if let Err(e) = write(istream, ostream, out_fmt) {
        eprintln!("{e:?}");
    }
}
