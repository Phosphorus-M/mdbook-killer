use std::{collections::HashMap, fs::read_to_string};

use leptos::{component, view, IntoView};
use leptos::prelude::{ClassAttribute, ElementChild, IntoAny};

use crate::{
    default_theme::{chapterpage::ChapterPage, chapters_navigator::ChaptersNavigator, custom_component::CustomComponent, layout::Layout},
    models::Chapter,
};

#[component]
pub fn Homepage(
    #[prop(optional)] chapter: Option<Chapter>,
    #[prop()] chapters: Vec<Chapter>,
    #[prop()] language: String,
) -> impl IntoView {
    let first_chapter = chapters.first().unwrap().clone();
    let chapter_body = read_to_string("./theme/chapter_body.html").ok();

    let main_content = if let Some(chapter_body) = chapter_body {
        let props = HashMap::<String, String>::new();

        view!{
            <div>
                <CustomComponent props=props content={chapter_body}  />
            </div>
        }
        .into_any()
    } else {
        view!{
            <div class="ml-52 px-6">
                <div class="flex w-full flex-row flex-1 items-center mt-6">
                    <ChapterPage chapter=chapter.unwrap_or(first_chapter) />
                </div>
            </div>
        }
        .into_any()
    };

    view! {
        <Layout is_home=true wide=false language=language.clone()>
            <ChaptersNavigator chapters=chapters.clone() language=language />
            {main_content}
        </Layout>
    }
}
