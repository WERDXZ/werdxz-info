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
            // Trigger when section is in the 10%-20% zone from the top of viewport
            options.set_root_margin("-10% 0px -80% 0px");
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
        <header>
            <div>
                <h1>"Jiqing Yang"</h1>
                <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                    {move || {
                        hero_content.get().map(|result| {
                            match result {
                                Ok(content) => view! {
                                    <p>{content.subtitle}</p>
                                    <p>{content.description}</p>
                                }.into_any(),
                                Err(_) => view! {
                                    <p>"Software Engineer"</p>
                                    <p>"Building scalable systems."</p>
                                }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </div>

            <nav aria-label="Main navigation">
                <a href="#about" class:active=move || active_section.get() == "about">"About"</a>
                <a href="#experience" class:active=move || active_section.get() == "experience">"Experience"</a>
                <a href="#projects" class:active=move || active_section.get() == "projects">"Projects"</a>
                {move || has_posts.get().then(|| view! {
                    <a href="#writing" class:active=move || active_section.get() == "writing">"Writing"</a>
                })}
            </nav>

            <footer>
                <address>
                    <a href="mailto:contact@werdxz.com">
                        "contact@werdxz.com"
                    </a>
                </address>

                <div class="social-links" aria-label="Social links">
                    <a href="https://linkedin.com/in/werdxz" target="_blank" rel="noopener noreferrer">
                        "LinkedIn"
                    </a>
                    <a href="https://github.com/werdxz" target="_blank" rel="noopener noreferrer">
                        "GitHub"
                    </a>
                    <a href="https://blog.werdxz.info" target="_blank" rel="noopener noreferrer">
                        "Blog"
                    </a>
                </div>

                <div class="mode-switcher" aria-label="Portfolio modes">
                    <a href="/swe" class:active=move || matches!(mode.get(), Mode::SoftwareEngineer)>
                        "SWE"
                    </a>
                    <a href="/rust" class:active=move || matches!(mode.get(), Mode::Rust)>
                        "Rust"
                    </a>
                    <a href="/student" class:active=move || matches!(mode.get(), Mode::Student)>
                        "Student"
                    </a>
                </div>
            </footer>
        </header>
    }
}
