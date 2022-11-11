use lineup::*;

fn main() {
    println!(
        "{:?}",
        FormatBuilder::new().span(4).anchor(Anchor::Right).build()
    );
}
