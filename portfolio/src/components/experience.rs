use leptos::prelude::*;
use crate::{get_featured_experience, types::{Experience, Mode}};

#[component]
pub fn ExperienceSection(mode: Signal<Mode>) -> impl IntoView {
    let experiences = Resource::new(move || mode.get(), |mode| get_featured_experience(mode));

    view! {
        <section id="experience" class="experience-section">
            <Suspense fallback=|| view! { <p class="loading" role="status" aria-live="polite">"Loading experience..."</p> }>
                {move || {
                    experiences.get().map(|result| {
                        match result {
                            Ok(experiences) => {
                                if experiences.is_empty() {
                                    view! { <p class="empty-state">"No experience to display yet."</p> }.into_any()
                                } else {
                                    view! {
                                        <div class="experience-timeline">
                                            {experiences
                                                .into_iter()
                                                .map(|exp| view! { <ExperienceItem experience=exp /> })
                                                .collect_view()}
                                        </div>
                                        <a
                                            href="https://resume.werdxz.info"
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="section-link"
                                        >
                                            "View Full Resume →"
                                        </a>
                                    }.into_any()
                                }
                            }
                            Err(e) => view! {
                                <p class="error-state">"Failed to load experience: " {e.to_string()}</p>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn ExperienceItem(experience: Experience) -> impl IntoView {
    let Experience {
        company,
        role,
        period,
        location,
        description,
        tags,
        redirect_url,
    } = experience;

    let redirect_url_clone = redirect_url.clone();
    let redirect_url_clone2 = redirect_url.clone();

    let item_click = move |_| {
        if let Some(url) = &redirect_url_clone {
            let _ = window().open_with_url_and_target(url, "_blank");
        }
    };

    let item_keypress = move |e: web_sys::KeyboardEvent| {
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
            tabindex="0"
            on:click=item_click
            on:keypress=item_keypress
        >
            <div class="experience-header">
                <div class="experience-title-group">
                    <h3 class="experience-role">{role}</h3>
                    <h4 class="experience-company">{company}</h4>
                </div>
                <div class="experience-meta">
                    <span class="experience-period">{period}</span>
                    {location.map(|loc| view! {
                        <span class="experience-location">{" • "}{loc}</span>
                    })}
                </div>
            </div>
            <p class="experience-description">{description}</p>
            <div class="experience-tags">
                {tags
                    .into_iter()
                    .map(|tag| view! { <span class="tag">{tag}</span> })
                    .collect_view()}
            </div>
        </article>
    }
}
