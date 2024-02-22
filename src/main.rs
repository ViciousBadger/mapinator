use clap::Parser;
use colored::Colorize;
use eyre::{Report, Result};
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};
use std::fs::File;
use tera::Tera;
use walkdir::{DirEntry, WalkDir};

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

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    path: Option<String>,
    outfile: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = args.path.unwrap_or(".".to_string());
    let outfile = args.outfile.unwrap_or("map.html".to_string());

    println!("Reading all .md files recursively in '{}'...", path.blue());

    let filtered_files = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_file() && f.path().extension().is_some_and(|ext| ext == "md"));

    fn try_parse(file: &DirEntry) -> Result<Entry, Report> {
        let str = std::fs::read_to_string(file.path())?;

        let matter = Matter::<YAML>::new();
        let parsed = matter.parse(&str);

        let front_matter: FrontMatter = parsed
            .data
            .ok_or(eyre::format_err!("No frontmatter found in file"))?
            .deserialize()?;

        Ok(Entry {
            lat: front_matter.lat,
            lng: front_matter.lng,
            title: front_matter.title,
            content: markdown::to_html(&parsed.content),
        })
    }

    let entries = filtered_files
        .filter_map(|file| match try_parse(&file) {
            Ok(entry) => Some(entry),
            Err(report) => {
                println!(
                    "{} {:?} {}",
                    "File".red(),
                    &file.path(),
                    "was skipped".red()
                );
                println!("{} {}", "  Reason:".red(), report.to_string().yellow());
                None
            }
        })
        .collect();

    println!();

    let data = Data { entries };

    if data.entries.len() > 0 {
        let mut tera = Tera::default();
        tera.add_raw_template("template", include_str!("./template.html"))?;

        let mut context = tera::Context::new();
        context.insert("data", &data);

        let file = File::create(&outfile)?;
        tera.render_to("template", &context, &file)?;

        println!(
            "Rendered HTML file '{}' with {} markers.",
            &outfile.blue(),
            &data.entries.len()
        );
    } else {
        println!("No valid entries found, doing nothing.");
    }
    Ok(())
}
