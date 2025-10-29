#[cfg(feature = "ssr")]
use axum::Extension;
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use worker::Env;

use crate::types::{AboutContent, BlogPost, Experience, HeroContent, Mode, Project};

/// Get featured projects from KV
#[server(GetFeaturedProjects)]
pub async fn get_featured_projects(mode: Mode) -> Result<Vec<Project>, ServerFnError> {
    #[cfg(feature = "ssr")]
    use crate::worker_helpers;
    #[cfg(feature = "ssr")]
    use leptos_axum::extract;

    // Extract the worker Env from request extensions
    let Extension(env): Extension<Arc<Env>> = extract().await?;

    // Get KV namespace
    let kv = worker_helpers::get_kv(&env, "KV")
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get KV namespace: {}", e)))?;

    // Get list of featured project IDs for this mode
    let key = format!("portfolio:featured_projects:{}", mode.to_tag());
    let ids_value = worker_helpers::kv_get_text(&kv, &key)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get featured projects list: {}", e)))?;

    let ids: Vec<String> = match ids_value {
        Some(json_str) => serde_json::from_str(&json_str).unwrap_or_default(),
        None => return Ok(vec![]),
    };

    // Fetch all projects in parallel
    let mut projects = Vec::new();
    for id in ids {
        let key = format!("portfolio:project:{}", id);
        if let Ok(Some(json_str)) = worker_helpers::kv_get_text(&kv, &key).await {
            if let Ok(project) = serde_json::from_str::<Project>(&json_str) {
                projects.push(project);
            }
        }
    }

    Ok(projects)
}

/// Get featured experience from KV
#[server(GetFeaturedExperience)]
pub async fn get_featured_experience(mode: Mode) -> Result<Vec<Experience>, ServerFnError> {
    #[cfg(feature = "ssr")]
    use crate::worker_helpers;
    #[cfg(feature = "ssr")]
    use leptos_axum::extract;

    let Extension(env): Extension<Arc<Env>> = extract().await?;

    let kv = worker_helpers::get_kv(&env, "KV")
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get KV namespace: {}", e)))?;

    // Get list of featured experience IDs for this mode
    let key = format!("portfolio:featured_experience:{}", mode.to_tag());
    let ids_value = worker_helpers::kv_get_text(&kv, &key)
        .await
        .map_err(|e| {
            ServerFnError::new(format!("Failed to get featured experience list: {}", e))
        })?;

    let ids: Vec<String> = match ids_value {
        Some(json_str) => serde_json::from_str(&json_str).unwrap_or_default(),
        None => return Ok(vec![]),
    };

    // Fetch all experiences in parallel
    let mut experiences = Vec::new();
    for id in ids {
        let key = format!("portfolio:experience:{}", id);
        if let Ok(Some(json_str)) = worker_helpers::kv_get_text(&kv, &key).await {
            if let Ok(experience) = serde_json::from_str::<Experience>(&json_str) {
                experiences.push(experience);
            }
        }
    }

    Ok(experiences)
}

/// Get featured blog posts from API
#[server(GetFeaturedPosts)]
pub async fn get_featured_posts(mode: Mode) -> Result<Vec<BlogPost>, ServerFnError> {
    #[cfg(feature = "ssr")]
    use crate::worker_helpers;
    #[cfg(feature = "ssr")]
    use leptos_axum::extract;

    let Extension(env): Extension<Arc<Env>> = extract().await?;

    let kv = worker_helpers::get_kv(&env, "KV")
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get KV namespace: {}", e)))?;

    // Get list of featured post slugs from KV for this mode
    let key = format!("portfolio:featured_posts:{}", mode.to_tag());
    let slugs_value = worker_helpers::kv_get_text(&kv, &key)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get featured posts list: {}", e)))?;

    let slugs: Vec<String> = match slugs_value {
        Some(json_str) => serde_json::from_str(&json_str).unwrap_or_default(),
        None => return Ok(vec![]),
    };

    if slugs.is_empty() {
        return Ok(vec![]);
    }

    // Get API base URL from environment variable
    let api_base_url = worker_helpers::get_var(&env, "API_BASE_URL")
        .await
        .unwrap_or_else(|| crate::constants::DEFAULT_API_BASE_URL.to_string());

    // Fetch posts from API
    use worker::send::SendFuture;

    let posts = SendFuture::new(async move {
        let client = reqwest::Client::new();
        let mut posts = Vec::new();

        for slug in slugs {
            let url = format!("{}/v1/posts/{}", api_base_url, slug);
            let result = async {
                let response = client.get(&url).send().await?;
                let post = response.json::<BlogPost>().await?;
                Ok::<BlogPost, reqwest::Error>(post)
            }.await;

            match result {
                Ok(post) => posts.push(post),
                Err(e) => {
                    // Log error but continue (graceful degradation)
                    leptos::logging::log!("Failed to fetch post {}: {}", slug, e);
                }
            }
        }

        posts
    }).await;

    Ok(posts)
}

/// Get hero content from KV
#[server(GetHeroContent)]
pub async fn get_hero_content(mode: Mode) -> Result<HeroContent, ServerFnError> {
    #[cfg(feature = "ssr")]
    use crate::worker_helpers;
    #[cfg(feature = "ssr")]
    use leptos_axum::extract;

    let Extension(env): Extension<Arc<Env>> = extract().await?;

    let kv = worker_helpers::get_kv(&env, "KV")
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get KV namespace: {}", e)))?;

    // Get hero content for this mode
    let key = format!("portfolio:hero_content:{}", mode.to_tag());
    let content_value = worker_helpers::kv_get_text(&kv, &key)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get hero content: {}", e)))?;

    match content_value {
        Some(json_str) => {
            serde_json::from_str::<HeroContent>(&json_str)
                .map_err(|e| ServerFnError::new(format!("Failed to parse hero content: {}", e)))
        }
        None => Err(ServerFnError::new("Hero content not found")),
    }
}

/// Get about content from KV
#[server(GetAboutContent)]
pub async fn get_about_content(mode: Mode) -> Result<AboutContent, ServerFnError> {
    #[cfg(feature = "ssr")]
    use crate::worker_helpers;
    #[cfg(feature = "ssr")]
    use leptos_axum::extract;

    let Extension(env): Extension<Arc<Env>> = extract().await?;

    let kv = worker_helpers::get_kv(&env, "KV")
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get KV namespace: {}", e)))?;

    // Get about content for this mode
    let key = format!("portfolio:about_content:{}", mode.to_tag());
    let content_value = worker_helpers::kv_get_text(&kv, &key)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get about content: {}", e)))?;

    match content_value {
        Some(json_str) => {
            serde_json::from_str::<AboutContent>(&json_str)
                .map_err(|e| ServerFnError::new(format!("Failed to parse about content: {}", e)))
        }
        None => Err(ServerFnError::new("About content not found")),
    }
}
