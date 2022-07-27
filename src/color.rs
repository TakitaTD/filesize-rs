use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub fn set_color(stdout: &mut StandardStream, color: Color) {
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .expect("cannot change terminal text color");
}
