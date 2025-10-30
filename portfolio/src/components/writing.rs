use crate::{get_featured_posts, types::{BlogPost, Mode}};
use leptos::prelude::*;

#[component]
pub fn WritingSection(mode: Signal<Mode>) -> impl IntoView {
    let posts = Resource::new(move || mode.get(), |mode| get_featured_posts(mode));

    view! {
        <Suspense fallback=|| view! { <></> }>
            {move || {
                posts.get().map(|result| {
                    match result {
                        Ok(posts) if !posts.is_empty() => {
                            view! {
                                <section id="writing" class="writing-section">
                                    <div class="posts-grid">
                                        {posts
                                            .into_iter()
                                            .map(|post| view! { <BlogPostCard post=post /> })
                                            .collect_view()}
                                    </div>
                                </section>
                            }.into_any()
                        }
                        _ => view! { <></> }.into_any()
                    }
                })
            }}
        </Suspense>
    }
}

#[component]
fn BlogPostCard(post: BlogPost) -> impl IntoView {
    let BlogPost {
        slug,
        title,
        summary,
        published_at,
        tags,
    } = post;

    let post_url = format!("{}/posts/{}", crate::constants::BLOG_BASE_URL, slug);

    view! {
        <a href=post_url target="_blank" rel="noopener noreferrer" class="card">
            <h3 class="post-title">{title}</h3>
            <p class="post-summary">{summary}</p>
            <div class="post-meta">
                <span class="post-date">{published_at}</span>
                <div class="post-tags">
                    {tags
                        .into_iter()
                        .map(|tag| view! { <span class="tag">{tag}</span> })
                        .collect_view()}
                </div>
            </div>
        </a>
    }
}
