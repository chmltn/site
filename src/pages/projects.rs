use crate::utils::page_meta::PageMeta;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Project(
    children: Children,
    name: &'static str,
    timeframe: &'static str,
    id: &'static str,
    #[prop(optional)] github: &'static str,
    #[prop(optional)] image: &'static str,
) -> impl IntoView {
    let cover_image = move || {
        (!image.is_empty()).then(|| {
            view! {
                <img class="project-cover" src={image} loading="lazy" />
            }
        })
    };

    let github_icon = move || {
        (!github.is_empty()).then(|| {
            view! {
                <a href={format!("https://github.com/{}", github)} target="_blank">
                    <img src="/github.svg" />
                    <span>{github}</span>
                </a>
            }
        })
    };

    view! {
        <li id={id}>
            <div class="project-cover-wrapper">
                {cover_image}
            </div>
            <div class="project-meta">
                <div class="project-header">
                    <h2>{name}</h2>
                    {github_icon}
                </div>
                <h3>{timeframe}</h3>
                {children()}
            </div>
        </li>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <PageMeta title="Projects - Caleb Hamilton" description="Caleb's projects" />

        <h1>"Projects"</h1>
        <p>"These are some of my favorite projects from over the years."</p>

        <ul class="projects-list">
            <Project
                name="Cosma"
                timeframe="2025 \u{2013} present"
                github="cosmasense/cosma"
                image="/cosma.webp"
                id="cosma"
            >
                <p>
                    "Cosma is an AI-powered local seach engine for your files."
                </p>
                <p>
                    "Starting as a project for UW Madison's 2025 Summer of AI Laboratory, Cosma's development "
                    "continued into the fall as we refined for production deployment and "
                    "developed a MacOS app to make file search more accessible."
                </p>
                <p>
                    "Cosma's main selling point is its extremely fast semantic search across all your files, "
                    "meaning you can describe the ideas and concepts within your files instead of exact keyword matches. "
                    "By default, it also processes all your files 100% on-device with local LLMs and embedded models."
                </p>
            </Project>
            <Project
                name="Prita"
                timeframe="2025 \u{2013} present"
                image="/prita.webp"
                id="prita"
            >
                <p>
                    "Prita is an in-development read-it-later and content organization tool built for speed and ease-of-use. "
                    "It's designed to stay as out of your way as possible, providing the simplest gateway to "
                    "consuming and organizing the content of your choosing."
                </p>
                <p>
                    "Right now, Prita is just a passion project developed almost entirely by me over the course of 2025. "
                    "I plan to release a beta when I finish the (rather large) MVP. "
                    "If this sounds interesting and you want to get notified when something actually happens, "
                    "you can add yourself to the mailing list at " <a href="https://prita.app">"prita.app"</a>
                    ". I haven't sent out a single email yet and honestly don't plan to, so don't worry about getting spammed."
                </p>
            </Project>
            <Project
                name="This site"
                timeframe="2023 \u{2013} present"
                github="chmltn/site"
                image="/blog.webp"
                id="this-site"
            >
                <p>
                    "This website was created as a markdown blogging engine in Rust. "
                    "It includes a client compiled to WebAssembly and an API server. "
                    "To build it, I used fine-grained reactivity and SSR to create a fast, robust full-stack web app. "
                </p>
                <p>
                    "I also used this project to learn how to use Docker. "
                    "I wrote a Dockerfile, which is automatically built with GitHub Actions whenever I push to the remote."
                </p>
                <p>
                    "See " <A href="/blog/making-a-blog">"my blog post"</A> " for more info on how I created it."
                </p>
            </Project>
            <Project
                name="Zupplin"
                timeframe="2021 \u{2013} 2023"
                github="chmltn/zupplin"
                image="/zupplin.webp"
                id="zupplin"
            >
                <p>
                    "Zupplin was a toy chat server I made for fun. "
                    "It was a fully-featured chat platform complete with profiles, group chats, friends, and more. "
                    "I created a lightweight front-end with React, Redux, JavaScript/TypeScript, and HTML/CSS to interact with "
                    "the back-end infrastructure I built and deployed using Python, Tornado, Docker, PostgreSQL, and other technologies."
                </p>
                <p>
                    "I also used this project to learn how to automate CI/CD using GitHub Actions. "
                    "I wrote workflows to triage issues and PRs, lint the codebase, and build the front-end."
                </p>
            </Project>
            <Project
                name="Clam"
                timeframe="2019 \u{2013} 2022"
                github="chmltn/Clam"
                image="/clam.webp"
                id="clam"
            >
                <p>
                    "Clam was a Discord bot written in Python that interacted with the Discord API. "
                    "It was originally created as a fun, simple testing bot "
                    "that I used to experiment with new programming concepts and learn different technologies. "
                    "Clam ended up being used by my friends for music, a starboard, moderation, and more. "
                </p>
                <p>
                    "While writing Clam, I learned how to use PostgreSQL and asyncpg. "
                    "I also learned web scraping and how to interact with a variety of REST APIs to provide extra functionality."
                </p>
            </Project>
        </ul>
    }
}
