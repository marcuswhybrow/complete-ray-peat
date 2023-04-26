mod markdown;

use std::fs;
use std::path::Path;
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use extract_frontmatter::{Extractor, config::Splitter::EnclosingLines};
use crate::markdown::timecode::InlineTimecode;
use crate::markdown::speaker::{SpeakerSection, TempSpeakerSection};


#[derive(Parser, Debug)]
#[command(name = "Ray Peat Rodeo Engine")]
#[command(author = "Marcus Whybrow <marcus@whybrow.uk>")]
#[command(about = "Builds Ray Peat Rodeo into HTML from source")]
#[command(long_about = None)]
struct Args {
    /// The input path containing markdown content
    #[arg(short, long, default_value_t = String::from("./content"))]
    input: String,

    /// The output path in which Ray Peat Rodeo should build HTML
    #[arg(short, long, default_value_t = String::from("./build"))]
    output: String,

    /// Whether files and directories inside of OUT_PATH should be deleted
    /// before building
    #[arg(short, long, default_value_t = false)]
    clean: bool,


    #[arg(short, long, default_value_t = String::from("./engine/templates"))]
    templates: String,
}


fn main() {

    // CLI Arguments and Options

    let args = Args::parse();
    let input = &Path::new(&args.input).canonicalize().unwrap();
    let templates = format!("{}/**/*", Path::new(&args.templates).canonicalize().unwrap().to_string_lossy());
    let output = &Path::new(&args.output).canonicalize().unwrap();

    println!("Building Ray Peat Rodeo");
    println!("Input: {:?}", input);
    println!("Templates: {:?}", templates);
    println!("Output: {:?}", output);

    if !output.exists() {
        println!("Creating directory");
        fs::create_dir(output).unwrap();
    } else {
        if args.clean {
            println!("Clean option enabled. \
                Deleting files and directories inside {:?}",
                output);

            for entry in fs::read_dir(output).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if entry.file_type().unwrap().is_dir() {
                    fs::remove_dir_all(path).unwrap();
                } else {
                    fs::remove_file(path).unwrap();
                }
            }
        }
    }

    // Templating

    let tera = match tera::Tera::new(templates.as_str()) {
        Ok(t) => t,
        Err(e) => panic!("Could not establish templates at {:?}\n{e}", templates),
    };
    
    let mut gcx = tera::Context::new();
    gcx.insert("global_project_link", "https://github.com/marcuswhybrow/ray-peat-rodeo");
    gcx.insert("global_contact_link", "https://raypeat.rodeo/contact");

    let render = |template, context: &tera::Context, path: &str| {
        let final_path = output.join(path);
        std::fs::create_dir_all(&final_path.parent().unwrap()).unwrap();
        tera.render_to(
            template,
            &context,
            std::fs::File::create(&final_path).unwrap(),
        ).unwrap();
        println!("Wrote {:?}", final_path);
    };

    // Render Specific Pages

    render("index.html", &gcx, "index.html");
    render("style.css", &gcx, "style.css");

    // Render Content

    let frontmatter_extractor = Extractor::new(EnclosingLines("---"));

    #[derive(Serialize, Deserialize, Debug)]
    struct Transcription {
        source: Option<String>,
        author: Option<String>,
        date: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Frontmatter {
        title: String,
        series: Option<String>,
        speakers: BTreeMap<String, String>, 
        source: String,
        source_duration: Option<String>,
        transcription: Option<Transcription>,
    }

    let markdown_parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(markdown_parser);
    markdown::timecode::add(markdown_parser);
    markdown::speaker::add(markdown_parser);

    for entry in fs::read_dir(input).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let text = std::fs::read_to_string(&path).unwrap();

        let (frontmatter_text, markdown) = frontmatter_extractor.extract(text.as_str());

        let frontmatter: Frontmatter = match serde_yaml::from_str(&frontmatter_text) {
            Ok(f) => f,
            Err(e) => panic!("Invalid YAML frontmatter in {:?}\n{e}", path),
        };

        let mut ast = markdown_parser.parse(markdown);

        ast.walk_post_mut(|node, _depth| {
            if let Some(timecode) = node.cast_mut::<InlineTimecode>() {
                timecode.url = match url::Url::parse(frontmatter.source.as_str()) {
                    Ok(url) => Some(url),
                    Err(e) => panic!("Malformed `source` field in YAML frontmatter in {:?}\n{e}", path),
                };
            } else if let Some(temp_speaker_section) = node.cast::<TempSpeakerSection>() {
                node.replace(SpeakerSection {
                    shortname: temp_speaker_section.shortname.clone(),
                    longname: match frontmatter.speakers.get(&temp_speaker_section.shortname) {
                        Some(ln) => ln.clone(),
                        None => panic!("Speaker shortname \"{}\" not found in \"speakers\" in YAML frontmatter in {:?}", temp_speaker_section.shortname, path),
                    },
                });
            }
        });

        let html = ast.render();
        let (_, slug) = path.file_stem().unwrap().to_str().unwrap().split_at(11);
        let out_name = &format!("{}/index.html", &slug);

        let mut cx = tera::Context::new();
        cx.insert("contents", html.as_str());
        cx.extend(gcx.clone());

        render("page.html", &cx, out_name);
    }

    println!("Done");
}
