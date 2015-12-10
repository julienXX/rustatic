extern crate hoedown;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;

use hoedown::Markdown;
use hoedown::renderer::Render;
use hoedown::renderer::html::{Html, Flags};

fn main() {
    match env::args().nth(1) {
        Some(file) => {
            let html = convert(file);
            write_file(html);
        }
        None => {
            println!("Usage: rustatic <path/to/file>");
            return;
        }
    };

}

fn convert(file: String) -> String {
    println!("Converting {}\n", file);

    let path = Path::new(&file);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(..)  => panic!("Can't find file."),
    };

    let mut s = String::new();
    file.read_to_string(&mut s);

    let input = Markdown::new(&s);
    let mut html = Html::new(Flags::empty(), 0);

    let result = html.render(&input);

    return result.to_str().unwrap().to_owned();
}

fn write_file(html: String) {
    let path = Path::new("test.html");

    let mut options = OpenOptions::new();
    options.read(true)
        .write(true)
        .create(true);

    let file = match options.open(&path) {
        Ok(file) => file,
        Err(..) => panic!("at the Disco"),
    };

    let mut writer = BufWriter::new(&file);
    writer.write_all(&html.into_bytes());
}
