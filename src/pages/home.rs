use crate::utils::page_meta::PageMeta;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <PageMeta title="Caleb Hamilton" description="Caleb's personal website" />

        <h1>"Hi, I'm Caleb!"</h1>

        <p>
            "I'm an engineer based in Madison, WI. "
            "I studied computer science and math at UW-Madison, and I currently work across the stack on backend, mobile, "
            "infrastructure, and web. I always love learning new languages, tools, and technologies, "
            "but I particularly enjoy thinking about scalable, distributed systems and full-stack web development. "
            "For example, this website is itself a "<A href="/projects#this-site">"full-stack Rust web app"</A>"."
        </p>
        <p>
            "You can find some of my favorite projects on my "
            <A href="/projects">"projects page"</A>
            ", see what I've been thinking about on "
            <A href="/blog">"my blog"</A>
            ", or browse all my public work on my "
            <a rel="external" href="https://github.com/chmltn">"GitHub"</a>"."
        </p>
    }
}
