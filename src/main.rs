extern crate hoedown;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use hoedown::Markdown;
use hoedown::renderer::Render;
use hoedown::renderer::html::{Html, Flags};

fn main() {
    match env::args().nth(1) {
        Some(file) => {
            convert(file);
        }
        None => {
            println!("Usage: rustatic <path/to/file>");
            return;
        }
    };

}

fn convert(file: String) {
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

    println!("{:?}", result.to_str().unwrap());
}
