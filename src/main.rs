extern crate hoedown;
extern crate rustc_serialize;
extern crate handlebars;

use std::env;
use std::io::prelude::*;
use std::fs;
use std::fs::{File,OpenOptions};
use std::io::BufWriter;
use std::path::{Path,PathBuf};

use hoedown::Markdown;
use hoedown::renderer::Render;
use hoedown::renderer::html::{Html, Flags};

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};

use handlebars::Handlebars;

mod server;

fn main() {
    match env::args().nth(1) {
        Some(file) => {
            setup().ok();

            let html = convert(&file);
            let content = render_layout(html);
            let html_file_path = write_file(file, content);

            server::run(html_file_path);
        }
        None => {
            println!("Usage: rustatic <path/to/file>");
            return;
        }
    };

}

fn setup() -> std::io::Result<()> {
    try!(fs::create_dir("./_site"));
    try!(fs::create_dir("./_posts"));
    try!(fs::create_dir("./_layouts"));
    Ok(())
}

fn convert(file: &String) -> String {
    println!("Converting {}\n", file);

    let path = Path::new(&file);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(..)  => panic!("Can't find file."),
    };

    let mut s = String::new();
    file.read_to_string(&mut s).ok();

    let input = Markdown::new(&s);
    let mut html = Html::new(Flags::empty(), 0);

    let result = html.render(&input);

    return result.to_str().unwrap().to_owned();
}

fn write_file(file_path: String, html: String) -> String {
    let source_path = Path::new(&file_path);
    let file_name = source_path.file_name().unwrap();

    let mut path = PathBuf::from("_site/");
    path.push(file_name);
    path.set_extension("html");

    let mut options = OpenOptions::new();
    options.read(true)
        .write(true)
        .create(true)
        .append(false);

    let file = match options.open(&path) {
        Ok(file) => file,
        Err(..) => panic!("at the Disco"),
    };

    let mut writer = BufWriter::new(&file);
    writer.write_all(&html.into_bytes()).ok();
    path.to_str().unwrap().to_string()
}

#[derive(Debug)]
struct Content {
    title: String,
    body: String
}

impl ToJson for Content {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("title".to_string(), self.title.to_json());
        m.insert("body".to_string(), self.body.to_json());
        m.to_json()
    }
}

fn render_layout(content: String) -> String {
    let mut handlebars = Handlebars::new();
    let path = Path::new("_layouts/default.hbs");

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(..)  => panic!("Can't find layout."),
    };

    let mut source = String::new();
    file.read_to_string(&mut source).ok();

    handlebars.register_template_string("default", source.to_string())
        .ok().unwrap();

    let data = Content {
        title: "Welcome to Rustatic!".to_string(),
        body: content,
    };

    handlebars.render("default", &data).unwrap()
}
