use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// Shared types: both the server and browser need these.

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    #[serde(rename = "documentId")]
    pub document_id: String,

    pub name: String,
    pub slug: String,

    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,

    #[serde(rename = "pageSize")]
    pub page_size: u32,

    #[serde(rename = "pageCount")]
    pub page_count: u32,

    pub total: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CategoryPage {
    pub categories: Vec<Category>,
    pub pagination: Pagination,
}

// These structures are only needed to decode Strapi's HTTP response.

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct StrapiResponse<T> {
    data: Vec<T>,
    meta: StrapiMeta,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct StrapiMeta {
    pagination: Pagination,
}

// Keep the function declaration available in BOTH builds.
// #[server] handles the server implementation / browser caller split.

#[server]
pub async fn get_categories(page: u32) -> Result<CategoryPage, ServerFnError> {
    use std::time::Duration;

    // Never trust browser-supplied arguments.
    if page == 0 {
        return Err(ServerFnError::new("Page must be at least 1."));
    }

    let base_url = std::env::var("STRAPI_BASE_URL")
        .map_err(|_| ServerFnError::new("CMS configuration is missing."))?;

    let token = std::env::var("STRAPI_API_TOKEN")
        .map_err(|_| ServerFnError::new("CMS configuration is missing."))?;

    let url = format!("{}/api/categories", base_url.trim_end_matches('/'));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|_| ServerFnError::new("Could not initialize the CMS client."))?;

    let page_string = page.to_string();

    let response = client
        .get(url)
        .bearer_auth(token)
        .query(&[
            ("pagination[page]", page_string.as_str()),
            ("pagination[pageSize]", "12"),
            ("pagination[withCount]", "true"),
            ("sort[0]", "name:asc"),
            ("fields[0]", "name"),
            ("fields[1]", "slug"),
            ("fields[2]", "description"),
            ("status", "published"),
        ])
        .send()
        .await
        .map_err(|_| ServerFnError::new("Could not reach the CMS. Please try again later."))?;

    if !response.status().is_success() {
        // Log only the status, not the token or raw CMS response.
        leptos::logging::error!(
            "Strapi categories request failed with HTTP {}",
            response.status()
        );

        return Err(ServerFnError::new(
            "Could not load categories. Please try again later.",
        ));
    }

    let payload = response
        .json::<StrapiResponse<Category>>()
        .await
        .map_err(|_| {
            leptos::logging::error!("Strapi categories response did not match the expected schema");

            ServerFnError::new("The CMS returned an unexpected response.")
        })?;

    Ok(CategoryPage {
        categories: payload.data,
        pagination: payload.meta.pagination,
    })
}
