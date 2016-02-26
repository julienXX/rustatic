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
    tldr: String,
    date: String,
    body: String,
    path: String,
    file_name: String,
}

impl ToJson for Document {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("title".to_string(), self.title.to_json());
        m.insert("tldr".to_string(), self.tldr.to_json());
        m.insert("date".to_string(), self.date.to_json());
        m.insert("body".to_string(), self.body.to_json());
        m.insert("path".to_string(), self.path.to_json());
        m.insert("file_name".to_string(), self.file_name.to_json());
        m.to_json()
    }
}

fn main() {
    let files = fs::read_dir("./_source").unwrap();

    let mut documents = Vec::new();

    for file in files {
        let path = file.unwrap().path().display().to_string();
        let document = parse_file(&path);

        let layout_path = Path::new("_layouts/default.hbs");
        let content = render_layout(&document, layout_path);

        let html_file_path = write_file(path, content);
        documents.push(document);
        // server::run(html_file_path);
    }

    if documents.len() >= 1 {
        add_to_index(documents);
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

    let content_clone = content.clone();
    let split = content_clone.split("---");
    let vec: Vec<&str> = split.collect();
    let header = vec.first().unwrap();
    content = vec.last().unwrap().to_string();

    let yaml_result = YamlLoader::load_from_str(header).unwrap();

    let title = yaml_result[0]["title"].as_str().unwrap_or("").to_owned();
    let tldr = yaml_result[0]["tldr"].as_str().unwrap_or("").to_owned();
    let date = yaml_result[0]["date"].as_str().unwrap_or("").to_owned();

    Document {
        title: title,
        tldr: tldr,
        date: date,
        body: markdown_to_html(content),
        path: path.display().to_string(),
        file_name: generate_link(path)
    }
}

fn generate_link(path: &Path) -> String {
    path.file_stem().unwrap().to_str().unwrap().to_owned()
}

fn markdown_to_html(content: String) -> String {
    let input = Markdown::new(&content);
    let mut html = Html::new(Flags::empty(), 0);
    let result = html.render(&input);

    return result.to_str().unwrap().to_owned();
}

fn render_layout(document: &Document, layout: &Path) -> String {
    let mut handlebars = Handlebars::new();

    let mut file = match File::open(layout) {
        Ok(file) => file,
        Err(..)  => panic!("Can't find layout."),
    };

    let mut source = String::new();
    file.read_to_string(&mut source).ok();

    handlebars.register_template_string("template", source.to_string())
        .ok().unwrap();

    println!("Rendering {:?}", document.path);

    handlebars.render("template", document).unwrap()
}

fn render_index(documents: &BTreeMap<String, Json>) -> String {
    let mut handlebars = Handlebars::new();
    let layout = Path::new("_layouts/index.hbs");

    let mut file = match File::open(layout) {
        Ok(file) => file,
        Err(..)  => panic!("Can't find layout."),
    };

    let mut source = String::new();
    file.read_to_string(&mut source).ok();

    handlebars.register_template_string("template", source.to_string())
        .ok().unwrap();

    println!("Rendering Index");

    handlebars.render("template", documents).unwrap()
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

fn add_to_index(documents: Vec<Document>) {
    let mut path = PathBuf::from("_site/");
    path.push("index");
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

    let mut data = BTreeMap::new();
    data.insert("documents".to_string(), documents.to_json());

    let html = render_index(&data);
    let mut writer = BufWriter::new(&file);
    writer.write_all(&html.into_bytes()).ok();
}
