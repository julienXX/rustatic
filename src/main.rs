extern crate hoedown;
extern crate rustc_serialize;
extern crate handlebars;
extern crate yaml_rust;

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
use yaml_rust::YamlLoader;

mod server;


#[derive(Debug)]
struct Document {
    title: String,
    date: String,
    body: String,
    path: String
}

impl ToJson for Document {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("title".to_string(), self.title.to_json());
        m.insert("date".to_string(), self.date.to_json());
        m.insert("body".to_string(), self.body.to_json());
        m.to_json()
    }
}

fn main() {
    let files = fs::read_dir("./_source").unwrap();

    for file in files {
        let path = file.unwrap().path().display().to_string();
        let document = parse_file(&path);
        let content = render_layout(&document);
        let html_file_path = write_file(path, content);
        // add_to_index();
        // server::run(html_file_path);
    }
}

fn parse_file(file: &String) -> Document {
    let path = Path::new(&file);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(..)  => panic!("Can't find file."),
    };

    let mut content = String::new();
    file.read_to_string(&mut content).ok();

    if content.contains("---") {
        let content2 = content.clone();
        let split = content2.split("---");
        let vec: Vec<&str> = split.collect();
        let header = vec.first().unwrap();
        let content = vec.last().unwrap();

        let yaml_result = YamlLoader::load_from_str(header).unwrap();

        let title = yaml_result[0]["title"].as_str().unwrap().to_owned();
        let date = yaml_result[0]["date"].as_str().unwrap().to_owned();

        Document {
            title: title,
            date: date,
            body: markdown_to_html(content.to_string()),
            path: path.display().to_string(),
        }
    } else {
        Document {
            title: "".to_owned(),
            date: "".to_owned(),
            body: markdown_to_html(content),
            path: path.display().to_string(),
        }
    }
}

fn markdown_to_html(content: String) -> String {
    let input = Markdown::new(&content);
    let mut html = Html::new(Flags::empty(), 0);
    let result = html.render(&input);

    return result.to_str().unwrap().to_owned();
}

fn render_layout(document: &Document) -> String {
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

    println!("Rendering {:?}", document.path);

    handlebars.render("default", document).unwrap()
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
