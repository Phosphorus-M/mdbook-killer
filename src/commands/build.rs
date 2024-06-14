use crate::default_theme::chapterpage::{ChapterPage, ChapterPageProps};
use crate::default_theme::homepage::{Homepage, HomepageProps};
use crate::models::lang_config::LanguageConfig;
use crate::models::Chapter;
use crate::renderer::ssg::Ssg;
use anyhow::{anyhow, Context, Result};
use std::fs::{self, ReadDir};
use std::path::Path;

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

    for lang in languages {
        let chapter_folder = fs::read_dir(format!("./src/{}", lang))?;
        println!("Reading in {:?}", chapter_folder);
        println!("--------");
        chapters.append(&mut charpters_from_folder(chapter_folder)?);
        println!("{:?}", chapters);
        println!("--------");
        println!("GENERACIÓN");
        println!("--------");

        let path = format!("./out/book/{lang}");

        let out = Path::new(&path);
        if !out.exists() {
            std::fs::create_dir_all(out).expect("Cannot create 'out' directory");
        }
        let ssg = Ssg::new(out);

        _ = generate_chapters(&ssg, chapters.clone(), lang.clone()).await;
    }
    _ = generate_homepage(&ssg, chapters, default_language).await;

    Ok(())
}

async fn generate_chapters<'a>(
    ssg: &Ssg<'a>,
    chapters: Vec<Chapter>,
    language: String
) -> Result<(), Box<dyn std::error::Error>> {
    let chapters_clone = chapters.clone();
    for chapter in chapters {
        let path = chapter.slug.clone().unwrap();
        let path = format!("{path}.html");

        let chapter_prop = Some(chapter.clone());
        let chapters_prop = chapters_clone.clone();
        let language_prop = language.clone();

        ssg.gen(path, || {
            Homepage(HomepageProps {
                chapter: chapter_prop,
                chapters: chapters_prop,
                language: language_prop
            })
        })
        .await?;
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

    for path in chapter_folder {
        let file = path?.path();
        let algo = fs::read_to_string(file.clone())?;
        let file = file
            .file_stem()
            .unwrap()
            .to_str()
            .with_context(|| "Could not convert path to str")?;
        if algo.starts_with("---") {
            let matter = Matter::<YAML>::new();
            let result = matter.parse_with_struct::<Chapter>(&algo);
            let Some(parsed_entity) = result else {
                println!("Error parsing file: {file:?}");
                continue;
            };
            let mut chapter: Chapter = parsed_entity.data;
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
