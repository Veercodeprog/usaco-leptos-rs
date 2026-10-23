use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
#[path = "header.rs"]
mod header;
use header::SiteHeader;
#[path = "home.rs"]
mod home;
use crate::categories::CategoriesPage;
use home::HomePagee;
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
            // Shared header: visible on every route.
            <SiteHeader />

            // Only this content changes when navigating.
            <main class="min-h-screen bg-slate-50">
                <Routes fallback=|| {
                    view! {
                        <div class="mx-auto max-w-7xl px-6 py-12">
                            <h1 class="text-3xl font-bold">"Page not found"</h1>
                            <p class="mt-3 text-slate-600">"This page has not been created yet."</p>
                        </div>
                    }
                }>
                    <Route path=StaticSegment("") view=HomePage />

                    <Route
                        path=StaticSegment("general")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Rust"
                                    description="Learn Rust fundamentals for problem solving."
                                    topics=vec![
                                        "Input and output",
                                        "Ownership and borrowing",
                                        "Vectors and strings",
                                        "Collections",
                                    ]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("bronze")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Bronze"
                                    description="Start with fundamental problem-solving techniques."
                                    topics=vec![
                                        "Simulation",
                                        "Complete search",
                                        "Sorting",
                                        "Greedy basics",
                                    ]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("silver")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Silver"
                                    description="Explore intermediate algorithms and data structures."
                                    topics=vec![
                                        "Prefix sums",
                                        "Binary search",
                                        "Two pointers",
                                        "Graph traversal",
                                    ]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("gold")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Gold"
                                    description="Study more advanced problem-solving techniques."
                                    topics=vec![
                                        "Dynamic programming",
                                        "Shortest paths",
                                        "Disjoint set union",
                                        "Tree algorithms",
                                    ]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("plat")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Platinum"
                                    description="Explore advanced algorithms and data structures."
                                    topics=vec![
                                        "Segment trees",
                                        "Range queries",
                                        "Advanced tree techniques",
                                    ]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("adv")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Advanced"
                                    description="Explore additional advanced topics."
                                    topics=vec![
                                        "Advanced dynamic programming",
                                        "String algorithms",
                                        "Mathematical algorithms",
                                    ]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("problems")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Problems"
                                    description="Browse practice problems by topic and difficulty."
                                    topics=vec![
                                        "Arrays and strings",
                                        "Linked lists",
                                        "Trees and graphs",
                                        "Dynamic programming",
                                    ]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("groups")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Groups"
                                    description="Organize your learning with study groups."
                                    topics=vec!["Your groups", "Group assignments"]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("editor")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Editor"
                                    description="Your content editor will appear here."
                                    topics=vec!["Create content", "Edit existing content"]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("contact")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Contact Us"
                                    description="Add your contact details or a contact form here."
                                    topics=vec![]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("settings")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Settings"
                                    description="Your account preferences will appear here."
                                    topics=vec![]
                                />
                            }
                        }
                    />

                    <Route
                        path=StaticSegment("login")
                        view=move || {
                            view! {
                                <ContentPage
                                    title="Sign In"
                                    description="Your sign-in form will appear here."
                                    topics=vec![]
                                />
                            }
                        }
                    />
                    <Route path=StaticSegment("categories") view=CategoriesPage />
                </Routes>
            </main>
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
    view! {
        <section class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
            <div class="border-b border-slate-200 pb-6">
                <h1 class="text-3xl font-bold tracking-tight text-slate-900">{title}</h1>

                <p class="mt-3 text-lg text-slate-600">{description}</p>
            </div>

            <div class="mt-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {topics
                    .into_iter()
                    .map(|topic| {
                        view! {
                            <article class="rounded-xl border border-slate-200 bg-white p-6">
                                <h2 class="text-lg font-semibold text-slate-900">{topic}</h2>
                                <p class="mt-2 text-sm text-slate-500">
                                    "Add lessons, explanations, and problems here."
                                </p>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
