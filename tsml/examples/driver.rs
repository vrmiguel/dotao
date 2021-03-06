use std::{fs, io};

use tsml::Groups;

fn main() -> io::Result<()> {
    // let path = "examples/simplest.tree";
    // let path = "examples/simple.tree";
    // let path = "examples/multiple_groups.tree";
    let path = "examples/dotao.tree";
    let text = fs::read_to_string(path)?;

    let groups = Groups::from_text(&text);
    dbg!(groups);

    Ok(())
}
