use leptos::prelude::*;
use crate::{get_hero_content, types::Mode};

#[component]
pub fn Hero(mode: Signal<Mode>, has_posts: Signal<bool>) -> impl IntoView {
    // Fetch hero content from KV
    let hero_content = Resource::new(move || mode.get(), |mode| get_hero_content(mode));

    #[allow(unused_variables)]
    let (active_section, set_active_section) = signal("about".to_string());

    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        use wasm_bindgen::JsCast;
        use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

        let callback = wasm_bindgen::closure::Closure::wrap(Box::new(move |entries: js_sys::Array| {
                // Find the section that is most visible (highest intersection ratio)
                let mut best_entry: Option<(String, f64)> = None;

                for entry in entries.iter() {
                    let entry: IntersectionObserverEntry = entry.dyn_into().unwrap();
                    if entry.is_intersecting() {
                        if let Some(id) = entry.target().get_attribute("id") {
                            let ratio = entry.intersection_ratio();
                            if let Some((_, best_ratio)) = &best_entry {
                                if ratio > *best_ratio {
                                    best_entry = Some((id, ratio));
                                }
                            } else {
                                best_entry = Some((id, ratio));
                            }
                        }
                    }
                }

                if let Some((id, _)) = best_entry {
                    set_active_section.set(id);
                }
            }) as Box<dyn FnMut(js_sys::Array)>);

            let options = IntersectionObserverInit::new();
            // Trigger when section enters top 30% of viewport (more lenient for top sections)
            options.set_root_margin("-30% 0px -50% 0px");
            // Use multiple thresholds for better tracking
            let thresholds = js_sys::Array::new();
            thresholds.push(&wasm_bindgen::JsValue::from_f64(0.0));
            thresholds.push(&wasm_bindgen::JsValue::from_f64(0.25));
            thresholds.push(&wasm_bindgen::JsValue::from_f64(0.5));
            thresholds.push(&wasm_bindgen::JsValue::from_f64(0.75));
            thresholds.push(&wasm_bindgen::JsValue::from_f64(1.0));
            options.set_threshold(&thresholds);

            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &options,
            ).unwrap();

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            // Observe all sections (writing section may or may not exist)
            // Note: about section needs to be at viewport top to trigger
            for id in ["about", "experience", "projects", "writing"] {
                if let Some(element) = document.get_element_by_id(id) {
                    observer.observe(&element);
                }
            }

            callback.forget();
    });

    view! {
        <header class="hero-sidebar">
            <div class="hero-header">
                <h1 class="hero-title">"Jiqing Yang"</h1>
                <Suspense fallback=|| view! { <p class="hero-role">"Loading..."</p> }>
                    {move || {
                        hero_content.get().map(|result| {
                            match result {
                                Ok(content) => view! {
                                    <p class="hero-role">{content.subtitle}</p>
                                    <p class="hero-description">{content.description}</p>
                                }.into_any(),
                                Err(_) => view! {
                                    <p class="hero-role">"Software Engineer"</p>
                                    <p class="hero-description">"Building scalable systems."</p>
                                }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </div>

            <nav class="hero-nav" aria-label="Main navigation">
                <a href="#about" class="nav-link" class:active=move || active_section.get() == "about">"About"</a>
                <a href="#experience" class="nav-link" class:active=move || active_section.get() == "experience">"Experience"</a>
                <a href="#projects" class="nav-link" class:active=move || active_section.get() == "projects">"Projects"</a>
                {move || has_posts.get().then(|| view! {
                    <a href="#writing" class="nav-link" class:active=move || active_section.get() == "writing">"Writing"</a>
                })}
            </nav>

            <footer class="hero-footer">
                <address class="hero-contact">
                    <a href="mailto:contact@werdxz.com" class="contact-email">
                        "contact@werdxz.com"
                    </a>
                </address>

                <div class="hero-social" aria-label="Social links">
                    <a href="https://linkedin.com/in/werdxz" target="_blank" rel="noopener noreferrer" class="social-link">
                        "LinkedIn"
                    </a>
                    <a href="https://github.com/werdxz" target="_blank" rel="noopener noreferrer" class="social-link">
                        "GitHub"
                    </a>
                    <a href="https://blog.werdxz.info" target="_blank" rel="noopener noreferrer" class="social-link">
                        "Blog"
                    </a>
                </div>
            </footer>
        </header>
    }
}
