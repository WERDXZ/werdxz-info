use leptos::prelude::*;
use crate::{get_featured_projects, types::{Mode, Project, ProjectLink}};

#[component]
pub fn ProjectsSection(mode: Signal<Mode>) -> impl IntoView {
    let projects = Resource::new(move || mode.get(), |mode| get_featured_projects(mode));

    view! {
        <section id="projects" class="projects-section">
            <Suspense fallback=|| view! { <p class="loading" role="status" aria-live="polite">"Loading projects..."</p> }>
                {move || {
                    projects.get().map(|result| {
                        match result {
                            Ok(projects) => {
                                if projects.is_empty() {
                                    view! { <p class="empty-state">"No projects to display yet."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="projects-grid">
                                            {projects
                                                .into_iter()
                                                .map(|project| view! { <ProjectCard project=project /> })
                                                .collect_view()}
                                        </div>
                                        <a
                                            href="https://github.com/werdxz"
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="section-link"
                                        >
                                            "View All Projects →"
                                        </a>
                                    }.into_any()
                                }
                            }
                            Err(e) => view! {
                                <p class="error-state">"Failed to load projects: " {e.to_string()}</p>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    let Project {
        title,
        description,
        tags,
        image_url: _,
        redirect_url,
        links,
    } = project;

    let redirect_url_clone = redirect_url.clone();
    let redirect_url_clone2 = redirect_url.clone();

    let card_click = move |_| {
        if let Some(url) = &redirect_url_clone {
            let _ = window().open_with_url_and_target(url, "_blank");
        }
    };

    let card_keypress = move |e: web_sys::KeyboardEvent| {
        if e.key() == "Enter" || e.key() == " " {
            e.prevent_default();
            if let Some(url) = &redirect_url_clone2 {
                let _ = window().open_with_url_and_target(url, "_blank");
            }
        }
    };

    let has_redirect = redirect_url.is_some();

    view! {
        <article
            class="card"
            class:clickable=has_redirect
            tabindex=move || if has_redirect { "0" } else { "-1" }
            on:click=card_click
            on:keypress=card_keypress
        >
            <h3 class="project-title">{title}</h3>
            <p class="project-description">{description}</p>
            <div class="project-tags">
                {tags
                    .into_iter()
                    .map(|tag| view! { <span class="tag">{tag}</span> })
                    .collect_view()}
            </div>
            <div class="project-links">
                {links
                    .into_iter()
                    .map(|link| view! { <ProjectLinkButton link=link /> })
                    .collect_view()}
            </div>
        </article>
    }
}

#[component]
fn ProjectLinkButton(link: ProjectLink) -> impl IntoView {
    let ProjectLink { label, url } = link;

    view! {
        <a
            href=url
            target="_blank"
            rel="noopener noreferrer"
            class="project-link"
            on:click=move |e| e.stop_propagation()
        >
            {label}
        </a>
    }
}
