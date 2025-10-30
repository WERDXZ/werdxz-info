use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    StaticSegment,
};

use crate::components::*;
use crate::get_featured_posts;
use crate::types::Mode;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
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
        <Title text="WERDXZ - Software Engineer Portfolio"/>
        <Meta name="description" content="Professional portfolio showcasing software engineering projects, experience, and technical writing."/>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>

        // content for this welcome page
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=|| view! { <Redirect path="/swe"/> }/>
                <Route path=StaticSegment("swe") view=HomePage/>
                <Route path=StaticSegment("rust") view=HomePage/>
                <Route path=StaticSegment("student") view=HomePage/>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Get mode from URL path - use Memo for derived computed value
    let location = leptos_router::hooks::use_location();
    let mode = Memo::new(move |_| {
        let path = location.pathname.get();
        let path_str = path.trim_start_matches('/');
        Mode::from_path(path_str)
    });

    // Fetch posts to determine if Writing section should be shown
    let posts = Resource::new(move || mode.get(), |mode| get_featured_posts(mode));
    let has_posts = Memo::new(move |_| {
        posts.get().map_or(false, |result| {
            result.as_ref().map_or(false, |posts| !posts.is_empty())
        })
    });

    view! {
        <Hero mode=mode.into() has_posts=has_posts.into()/>
        <main>
            <AboutSection mode=mode.into()/>
            <ExperienceSection mode=mode.into()/>
            <ProjectsSection mode=mode.into()/>
            <WritingSection mode=mode.into()/>
            <footer>
                <p>"© 2025 werdxz"</p>
            </footer>
        </main>
    }
}