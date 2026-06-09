use futures::{Stream, StreamExt};
use leptos::prelude::*;
use lol_html::{element, HtmlRewriter, Settings};
use std::{error::Error, pin::Pin};
use tokio::task;

use crate::default_theme::meta;

pub async fn render<F, V>(
    view: F,
    additional_context: impl FnOnce() + 'static,
) -> String
where
    F: FnOnce() -> V + 'static,
    V: IntoView,
{
    let local = task::LocalSet::new();
    local
        .run_until(async move {
            let shell_ctx = meta::ShellCtx::new();
            let shell_ctx2 = shell_ctx.clone();

            let owner = Owner::new();
            let stream = owner.with(|| {
                provide_context(shell_ctx);
                additional_context();
                view().into_view().to_html_stream_in_order()
            });
            let stream = Box::pin(stream);
            let out = clean_leptos_ssr(stream).await.unwrap();

            shell_ctx2.render(out)
        })
        .await
}

async fn clean_leptos_ssr(
    mut stream: Pin<Box<impl Stream<Item = String>>>,
) -> Result<String, Box<dyn Error>> {
    let mut output = vec![];

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("script:nth-of-type(1)", |el| {
                el.remove();
                Ok(())
            })],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );

    while let Some(chunk) = stream.next().await {
        rewriter.write(chunk.as_bytes())?;
    }
    rewriter.end()?;

    Ok(String::from_utf8(output)?)
}
