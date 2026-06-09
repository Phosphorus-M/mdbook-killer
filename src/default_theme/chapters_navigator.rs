use std::{collections::HashMap, env::current_dir, fs::read_to_string, path::Path};
use leptos::{component, view, IntoView};
use leptos::prelude::{ClassAttribute, CollectView, ElementChild, IntoAny, RenderHtml};
use crate::models::Chapter;
use super::custom_component::CustomComponent;



#[component]
pub fn ChaptersNavigator(
    #[prop()] chapters: Vec<Chapter>,
    #[prop()] language: String,
) -> impl IntoView {
    println!("{:?}", current_dir());
    let chapter_navigator = read_to_string("./theme/chapter_navigator.html").ok();
    let chapter_navigator_item = read_to_string("./theme/chapter_navigator_item.html").ok();

    let props = chapters.clone().into_iter().map(|chapter| {
        let link = if !language.is_empty() {
            format!("/{}.html", chapter.slug.unwrap())
        } else {
            format!("{}/{}.html", language.clone(), chapter.slug.unwrap())
        };

        (link, chapter.title)
    })
    .collect::<HashMap<String, String>>();

    let chapter_navigator_items = if let Some(chapter_navigator_item) = chapter_navigator_item {
        props
            .into_iter()
            .map(|(link, title)| {
                let mut custom_prop = HashMap::<String, String>::new();
                custom_prop.insert("link".to_string(), link);
                custom_prop.insert("title".to_string(), title);

                view!{
                    <CustomComponent props=custom_prop content=chapter_navigator_item.clone()  />
                }
            })
            .collect_view()
            .into_any()
    }else {
        props
            .into_iter()
            .map(|(link, title)| {
                view! {
                    <a href=link >{title}</a>
                }
            })
            .collect_view()
            .into_any()
    };

    let chapter_navigator_items = chapter_navigator_items.to_html();
    println!("Existe o no? {:?}", chapter_navigator);

    let mut props = HashMap::<String,String>::new();
    props.insert("links".to_string(), chapter_navigator_items.to_string());

    let navigator_view = if let Some(chapter_navigator) = chapter_navigator {
        view!{
            <nav>
                <CustomComponent props=props content=chapter_navigator  />
                "Cosa"
            </nav>
        }
        .into_any()
    }else{
        view!{
            <nav class="dark:bg-[#101010] fixed left-0 min-w-52 border-r border-gray-700 h-full py-2">
            {
                chapters.into_iter().map(|chapter| {
                    let link = if !language.is_empty() {
                        format!("/{}.html", chapter.slug.unwrap())
                    }else {
                        format!("{}/{}.html", language.clone(), chapter.slug.unwrap())
                    };

                    view! {
                    <div class="px-2 py-1">
                        <a href=link >{chapter.title}</a>
                        "XD"
                    </div>
                    }
                }).collect_view()
            }
            </nav>
        }
        .into_any()
    };

    view!{
        <>
            {navigator_view}
        </>
    }
}