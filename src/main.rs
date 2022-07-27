use clap::Parser;
use std::{fs, io::Write, path::Path};
mod color;
use termcolor::{Color, ColorChoice, StandardStream};

#[derive(Parser, Debug)]
struct CliArgs {
    file_path: Option<String>,
}

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    color::set_color(&mut stdout, Color::White);

    let cli_args = CliArgs::parse();
    let dir = cli_args.file_path.unwrap_or(String::from("."));

    if !Path::new(&dir).is_dir() {
        color::set_color(&mut stdout, Color::Red);
        write!(stdout, "error: ");
        color::set_color(&mut stdout, Color::White);
        writeln!(
            stdout,
            "it seems that the directory you specified, is not a directory! :("
        );
        std::process::exit(1)
    }

    for file in fs::read_dir(dir).unwrap() {
        println!(
            "{} is {:#?} bytes",
            file.as_ref().unwrap().path().display(),
            fs::metadata(file.unwrap().path()).unwrap().len()
        );
    }
}
