use futures::{executor::block_on, Future};
use leptos::{component, IntoView};

#[component]
/// Async wraps an async function that returns an IntoView into a Suspense.
///
/// This is useful when used together with async ssr that will wait on the provided async function
/// to render the final view.
pub fn Async<V, F, Fut>(view: F) -> impl IntoView
where
    V: IntoView + 'static,
    Fut: Future<Output = V> + 'static,
    F: Fn() -> Fut + 'static,
{
    // IDK how this was working before, temporal solution here:
    block_on(view())
}

