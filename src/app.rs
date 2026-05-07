use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::*,
    path,
};

use crate::pages::{
    blog::Blog, home::Home, not_found::NotFound, post::BlogPost, projects::Projects,
};

static SITE_TITLE: &'static str = "Caleb Hamilton";

const THEME_SCRIPT: &str = r#"(function(){var p=localStorage.getItem('theme')||'auto';var r=p==='auto'?(matchMedia('(prefers-color-scheme:light)').matches?'light':'dark'):p;document.documentElement.dataset.theme=r;document.documentElement.dataset.preference=p;matchMedia('(prefers-color-scheme:light)').addEventListener('change',function(e){if((localStorage.getItem('theme')||'auto')==='auto')document.documentElement.dataset.theme=e.matches?'light':'dark'});})()"#;

const ICON_LIGHT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2.25a.75.75 0 0 1 .75.75v2.25a.75.75 0 0 1-1.5 0V3a.75.75 0 0 1 .75-.75ZM7.5 12a4.5 4.5 0 1 1 9 0 4.5 4.5 0 0 1-9 0ZM18.894 6.166a.75.75 0 0 0-1.06-1.06l-1.591 1.59a.75.75 0 1 0 1.06 1.061l1.591-1.59ZM21.75 12a.75.75 0 0 1-.75.75h-2.25a.75.75 0 0 1 0-1.5H21a.75.75 0 0 1 .75.75ZM17.834 18.894a.75.75 0 0 0 1.06-1.06l-1.59-1.591a.75.75 0 1 0-1.061 1.06l1.59 1.591ZM12 18a.75.75 0 0 1 .75.75V21a.75.75 0 0 1-1.5 0v-2.25A.75.75 0 0 1 12 18ZM7.758 17.303a.75.75 0 0 0-1.061-1.06l-1.591 1.59a.75.75 0 0 0 1.06 1.061l1.591-1.59ZM6 12a.75.75 0 0 1-.75.75H3a.75.75 0 0 1 0-1.5h2.25A.75.75 0 0 1 6 12ZM6.697 7.757a.75.75 0 0 0 1.06-1.06l-1.59-1.591a.75.75 0 0 0-1.061 1.06l1.59 1.591Z"/></svg>"#;

const ICON_DARK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path fill-rule="evenodd" d="M9.528 1.718a.75.75 0 0 1 .162.819A8.97 8.97 0 0 0 9 6a9 9 0 0 0 9 9 8.97 8.97 0 0 0 3.463-.69.75.75 0 0 1 .981.98 10.503 10.503 0 0 1-9.694 6.46c-5.799 0-10.5-4.7-10.5-10.5 0-4.368 2.667-8.112 6.46-9.694a.75.75 0 0 1 .818.162Z" clip-rule="evenodd"/></svg>"#;

const ICON_AUTO: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><rect width="256" height="256" fill="none"/><path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24ZM40,128a88.1,88.1,0,0,1,88-88V216A88.1,88.1,0,0,1,40,128Z" fill="currentColor"/></svg>"#;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" id="leptos" href="/pkg/calebhamilton_org.css"/>
                <script inner_html=THEME_SCRIPT />
                <title>{SITE_TITLE}</title>

                <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
                <link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
                <link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
                <link rel="manifest" href="/site.webmanifest" />
                <meta name="msapplication-TileColor" content="#272838" />
                <meta name="theme-color" content="#272838" />

                <Script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js" />
                <link rel="preconnect" href="https://rsms.me/" />
                <link rel="stylesheet" href="https://rsms.me/inter/inter.css" />

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        //  TODO: add real error handling
        <Router>
        <div class="app">
                <header>
                    <nav>
                        <div class="logo">
                            <A exact=true href="/">"Caleb Hamilton"</A>
                        </div>
                        <ul>
                            <li>
                                <A href="blog">"blog"</A>
                            </li>
                            <li>
                                <A href="projects">"projects"</A>
                            </li>
                        </ul>
                    </nav>
                </header>

                <main>
                    <Routes fallback=move || {view!{ <NotFound />}.into_view()}>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/projects") view=Projects />
                        <Route path=path!("/blog") view=Blog />
                        <Route
                            path=path!("/blog/:id")
                            view=BlogPost
                            ssr=leptos_router::SsrMode::Async
                        />
                    </Routes>
                </main>
                <footer>
                    <ul>
                        <li>
                            <a rel="external" href="https://github.com/chmltn" target="_blank">
                                <img src="/github.svg" />"chmltn"
                            </a>
                        </li>
                        <li>
                            <a rel="external" href="https://www.linkedin.com/in/calebthamilton/" target="_blank">
                            <img src="/linkedin.svg" />"calebthamilton"
                        </a>
                        </li>
                        <li>
                            <ThemeToggle />
                        </li>
                    </ul>
                </footer>
            </div>
        </Router>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let toggle_theme = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = js_sys::eval(r#"
                var c = document.documentElement.dataset.preference || 'auto';
                var n = c === 'auto' ? 'light' : c === 'light' ? 'dark' : 'auto';
                localStorage.setItem('theme', n);
                document.documentElement.dataset.preference = n;
                document.documentElement.dataset.theme = n === 'auto' ?
                    (matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark') : n;
            "#);
        }
    };

    view! {
        <button class="theme-toggle" on:click=toggle_theme aria-label="Toggle theme">
            <span class="icon-light" inner_html=ICON_LIGHT />
            <span class="icon-dark" inner_html=ICON_DARK />
            <span class="icon-auto" inner_html=ICON_AUTO />
        </button>
    }
}
