mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut seoul = ThemeBuilder::new("Seoul".to_string(), Type::Dark);
    imp::add_rules(&mut seoul, &palette);
    seoul.build().save()?;

    Ok(())
}
