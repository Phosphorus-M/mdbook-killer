use std::{collections::HashMap, sync::{Arc, Mutex}};

use leptos::{
    IntoView, component, prelude::*, view
};

#[component]
pub fn Html(
    #[prop(into)] mut attrs: Attrs,
    #[prop(optional,into, default= "".to_string())] class: String,
) -> impl IntoView {
    let ctx = expect_context::<ShellCtx>();
    let mut class = Attrs::from(vec![("class", class.as_str())]);
    ctx.body_attrs.lock().unwrap().append(&mut class);
    ctx.html_attrs.lock().unwrap().append(&mut attrs);
}

#[component]
pub fn Head(children: Children) -> impl IntoView {
    let ctx = expect_context::<ShellCtx>();
    ctx.head_els.lock().unwrap().push(children().to_html());
}

#[component(transparent)]
pub fn Dedup(#[prop(into)] key: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<ShellCtx>();
    let mut map = ctx.deduped_head_els.lock().unwrap();
    map.entry(key).or_insert_with(|| children().to_html());
}

#[derive(Clone)]
/// `ShellCtx` holds all the elements that will be rendered to the <head> of the page.
/// It can be modified by any component by accessing the context, but it's suggested to be used in
/// conjunction with the exported components <Dedup />, <Title />, <Html />, ....
pub struct ShellCtx {
    head_els: Arc<Mutex<Vec<String>>>,
    deduped_head_els: Arc<Mutex<HashMap<String, String>>>,
    html_attrs: Arc<Mutex<Attrs>>,
    body_attrs: Arc<Mutex<Attrs>>,
}

impl ShellCtx {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn render(self, inner_body: String) -> String {
        let head = {
            let head_els = self.head_els.lock().unwrap().clone();
            let deduped_head_els = self.deduped_head_els.lock().unwrap().clone();
            let mut head = String::new();
            for item in head_els {
                head.push_str(&item);
            }
            for item in deduped_head_els.values() {
                head.push_str(item);
            }
            head
        };

        format!(
            "<!DOCTYPE html><html {}><head>{}</head><body {}>{}</body></html>",
            self.html_attrs.lock().unwrap().render(),
            head,
            self.body_attrs.lock().unwrap().render(),
            inner_body.trim(),
        )
    }
}

impl Default for ShellCtx {
    fn default() -> Self {
        Self {
            head_els: Arc::new(Mutex::new(Vec::new())),
            deduped_head_els: Arc::new(Mutex::new(HashMap::new())),
            html_attrs: Arc::new(Mutex::new(Attrs::default())),
            body_attrs: Arc::new(Mutex::new(Attrs::default())),
        }
    }
}

/// Attrs is a list of attributes.
#[derive(Default)]
pub struct Attrs {
    pub attrs: Vec<(String, String)>,
}

impl Attrs {
    #[must_use]
    pub fn new() -> Self {
        Self { attrs: vec![] }
    }

    #[must_use]
    pub fn render(&self) -> String {
        self.attrs
            .iter()
            .map(|(k, v)| format!("{k}=\"{v}\""))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn append(&mut self, other: &mut Self) {
        self.attrs.append(&mut other.attrs);
    }
}

impl From<Vec<(&str, &str)>> for Attrs {
    fn from(attrs: Vec<(&str, &str)>) -> Self {
        Self {
            attrs: attrs
                .iter()
                .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
                .collect(),
        }
    }
}

impl From<Vec<(String, String)>> for Attrs {
    fn from(attrs: Vec<(String, String)>) -> Self {
        Self { attrs }
    }
}
