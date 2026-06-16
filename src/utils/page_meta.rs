use leptos::{prelude::*, text_prop::TextProp};
use leptos_meta::*;

#[component]
pub fn PageMeta(
    #[prop(into)] title: TextProp,
    #[prop(into)] description: TextProp,
    // og:title defaults to the document title, but pages (e.g. blog posts) can
    // override it to drop the site-name suffix from social cards.
    #[prop(into, optional)] og_title: Option<TextProp>,
) -> impl IntoView {
    let og_title = og_title.unwrap_or_else(|| title.clone());
    view! {
        <Title text={title.clone()}/>
        <Meta name="description" content={description.clone()} />
        <Meta property="og:title" content={og_title} />
        <Meta property="og:description" content={description} />
    }
}
