use leptos::prelude::*;
use crate::get_about_content;

#[component]
pub fn AboutSection(mode: Signal<crate::types::Mode>) -> impl IntoView {
    // Fetch about content from KV
    let about_content = Resource::new(move || mode.get(), |mode| get_about_content(mode));

    view! {
        <section id="about" class="about-section">
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || {
                    about_content.get().map(|result| {
                        match result {
                            Ok(content) => content.paragraphs.into_iter()
                                .map(|p| view! { <p>{p}</p> })
                                .collect_view()
                                .into_any(),
                            Err(_) => view! {
                                <p>"Content unavailable."</p>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </section>
    }
}
