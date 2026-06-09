use crate::default_theme::chapterpage::{ChapterPage, ChapterPageProps};
use crate::default_theme::custom_component::{CustomComponent, CustomComponentProps};
use crate::default_theme::homepage::{Homepage, HomepageProps};
use crate::models::lang_config::LanguageConfig;
use crate::models::Chapter;
use crate::renderer::ssg::Ssg;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::fs::{self, read_to_string, ReadDir};
use std::path::{Path, PathBuf};

use gray_matter::engine::YAML;
use gray_matter::Matter;
use tailwind_css::TailwindBuilder;

static CSS_FILE: &'static str = include_str!("../../leptos_start.css");

pub async fn execute(
    default_language: Option<String>,
    languages: Option<Vec<String>>,
) -> Result<()> {
    println!("{languages:?}");

    let languages = languages.or(Some(vec!["".to_string()])).unwrap();

    let out = Path::new("./out/book");
    if !out.exists() {
        std::fs::create_dir_all(out).expect("Cannot create 'out' directory");
    }

    let ssg = Ssg::new(out);
    std::fs::write("./out/book/style.css", CSS_FILE)?;

    let mut chapters = Vec::with_capacity(10);
    let custom_component = read_to_string("./theme/chapter.html").ok();

    for lang in languages {
        let chapter_folder = fs::read_dir(format!("./src/{}", lang))?;
        println!("Reading in {:?}", chapter_folder);
        println!("--------");
        chapters.append(&mut charpters_from_folder(chapter_folder)?);
        // println!("{:?}", chapters);
        println!("--------");
        println!("GENERACIÓN");
        println!("--------");

        let path = format!("./out/book/{lang}");

        let out = Path::new(&path);
        if !out.exists() {
            std::fs::create_dir_all(out).expect("Cannot create 'out' directory");
        }
        let ssg = Ssg::new(out);

        _ = generate_chapters(&ssg, chapters.clone(), lang.clone(), custom_component.clone()).await;
    }
    _ = generate_homepage(&ssg, chapters, default_language).await;

    Ok(())
}

async fn generate_chapters<'a>(
    ssg: &Ssg<'a>,
    chapters: Vec<Chapter>,
    language: String, 
    custom_component: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    

    let chapters_clone = chapters.clone();
    for chapter in chapters {
        let path = chapter.slug.clone().unwrap();
        let path = format!("{path}.html");

        let chapter_prop = Some(chapter.clone());
        let chapters_prop = chapters_clone.clone();
        let language_prop = language.clone();
        
        if let Some(custom_component) = custom_component.clone() {
            let mut props = HashMap::<String, String>::new();
            props.insert("cosa".to_string(), "algooo".to_string());

            ssg.gen(path, || CustomComponent(CustomComponentProps{
                content: custom_component,
                props
            })).await?;
        }else {
            ssg.gen(path, || Homepage(HomepageProps{
                chapter:  chapter_prop,
                chapters: chapters_prop,
                language: language_prop
            })).await?;
        }
    }

    Ok(())
}

async fn generate_homepage<'a>(
    ssg: &Ssg<'a>,
    chapters: Vec<Chapter>,
    default_language: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    ssg.gen("index.html".to_owned(), || {
        Homepage(HomepageProps {
            chapters,
            chapter: None,
            language: default_language.unwrap_or("".to_string())
        })
    })
    .await?;

    Ok(())
}

fn charpters_from_folder(chapter_folder: ReadDir) -> Result<Vec<Chapter>> {
    let mut chapters = Vec::with_capacity(10);

    println!("Reading chapters from folder...");
    for path in chapter_folder {
        let file = path?.path();
        println!("Reading file: {file:?}");
        if file.is_dir() {
            println!("Reading directory: {file:?}");
            if !chapter_folder_detection(file.clone()) {
                continue;
            }
            let sub_folder = fs::read_dir(&file)?;
            chapters.append(&mut charpters_from_folder(sub_folder)?);
            continue;
        }
        let algo = fs::read_to_string(file.clone())?;
        let file = file
            .file_stem()
            .expect("Could not get file stem")
            .to_str()
            .with_context(|| "Could not convert path to str")?;
        if algo.starts_with("---") {
            let matter = Matter::<YAML>::new();
            let parsed_entity = match matter.parse::<Chapter>(&algo) {
                Ok(parsed_entity) => parsed_entity,
                Err(error) => {
                    println!("Error parsing file {file:?}: {error}");
                    continue;
                }
            };
            let Some(mut chapter) = parsed_entity.data else {
                println!("Error parsing file: {file:?}");
                continue;
            };
            chapter.content = Some(parsed_entity.content);

            chapter.slug.get_or_insert(file.to_string());

            chapters.push(chapter);
        } else {
            let title = algo.clone();
            let title = title
                .lines()
                .next()
                .ok_or(anyhow!("No se pudo obtener un titulo"))?;

            let chapter = Chapter {
                title: title.to_string(),
                content: Some(algo),
                slug: Some(file.to_string()),
            };
            chapters.push(chapter);
        }
    }

    Ok(chapters)
}


/// this method check if the path is a folder, if it is and the folder does not contain markdown files, it will
///  copy all the content of the folder to the output, to be used for assets like images, css, js, etc.
fn chapter_folder_detection(path: PathBuf) -> bool {
    let mut has_md_files = false;
    for entry in fs::read_dir(&path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            has_md_files = true;
            break;
        }
    }
    if !has_md_files {
        // copy the folder to the output
        let out_path = Path::new("./out/book").join(path.file_name().unwrap());
        fs::create_dir_all(&out_path).unwrap();
        for entry in fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                fs::create_dir_all(out_path.join(path.file_name().unwrap())).unwrap();
                for entry in fs::read_dir(&path).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() {
                        let file_name = path.file_name().unwrap();
                        fs::copy(&path, out_path.join(file_name)).unwrap();
                    }
                }
                continue;
            }
            let file_name = path.file_name().unwrap();
            fs::copy(&path, out_path.join(file_name)).unwrap();
        }
    }
    return has_md_files;
}