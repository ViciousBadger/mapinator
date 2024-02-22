use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};
use std::fs::File;
use tera::Tera;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct FrontMatter {
    title: String,
    lat: f32,
    lng: f32,
}

#[derive(Serialize)]
struct Data {
    entries: Vec<Entry>,
}

#[derive(Serialize)]
struct Entry {
    lat: f32,
    lng: f32,
    title: String,
    content: String,
}

fn main() -> tera::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };

    println!("Reading .md files in '{}'...", path);

    let filtered_files = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_file() && f.path().extension().is_some_and(|ext| ext == "md"));

    let matter = Matter::<YAML>::new();

    let entries = filtered_files
        .map(|file| {
            let str = std::fs::read_to_string(file.path()).unwrap();
            let parsed = matter.parse(&str);

            let front_matter: FrontMatter = parsed.data.unwrap().deserialize().unwrap();
            Entry {
                lat: front_matter.lat,
                lng: front_matter.lng,
                title: front_matter.title,
                content: markdown::to_html(&parsed.content),
            }
        })
        .collect();

    let data = Data { entries };

    let mut tera = Tera::default();
    tera.add_raw_template("template", include_str!("./template.html"))?;

    let mut context = tera::Context::new();
    context.insert("data", &data);

    let file = File::create("render.html")?;
    tera.render_to("template", &context, &file)?;

    println!("Rendered HTML file with {} markers.", &data.entries.len());

    Ok(())
}
