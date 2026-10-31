use crate::components::Navbar;
use crate::layouts::content_layout::ContentLayout;
use crate::layouts::main_layout::MainLayout;
use crate::pages::categories::CategoriesPage;
use crate::pages::home::HomePagee;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

// #[component]
// pub fn App() -> impl IntoView {
//     // Provides context that manages stylesheets, titles, meta tags, etc.
//     provide_meta_context();
//
//     view! {
//         // injects a stylesheet into the document <head>
//         // id=leptos means cargo-leptos will hot-reload this stylesheet
//         <Stylesheet id="leptos" href="/pkg/usaco-clone.css" />
//         // <link data-trunk rel="taliwind-css" href="style/input.css" />
//         // <Stylesheet href="/tailwind.css" />
//         <SiteHeader />
//         // sets the document title
//         <Title text="Welcome to Leptos" />
//
//         // content for this welcome page
//         <Router>
//             <main>
//                 <Routes fallback=|| "Page not found.".into_view()>
//                     <Route path=StaticSegment("") view=HomePage />
//                 </Routes>
//             </main>
//         </Router>
//     }
// }

/// Renders the home page of your application.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/usaco-clone.css" />
        <Title text="My Guide" />

        <Router>
            <Navbar />

            <Routes fallback=|| "Page not found.".into_view()>

                <Route
                    path=StaticSegment("general")
                    view=move || {
                        view! {
                            <MainLayout sidebar_items=vec![
                                "Input and output",
                                "Ownership and borrowing",
                                "Vectors and strings",
                                "Collections",
                            ]>
                                <ContentLayout
                                    title="Rust"
                                    description="Learn Rust fundamentals for problem solving."
                                    topics=vec![
                                        "Input and output",
                                        "Ownership and borrowing",
                                        "Vectors and strings",
                                        "Collections",
                                    ]
                                />
                            </MainLayout>
                        }
                    }
                />

            </Routes>
        </Router>
    }
}

/// Keep your original homepage wrapper, but render the component from home.rs.
#[component]
fn HomePage() -> impl IntoView {
    view! { <HomePagee /> }
}

/// Temporary reusable content page.
/// Replace these topic lists with your real content later.

#[component]
fn ContentPage(
    title: &'static str,
    description: &'static str,
    topics: Vec<&'static str>,
) -> impl IntoView {
    view! { <ContentLayout title=title description=description topics=topics /> }
}
