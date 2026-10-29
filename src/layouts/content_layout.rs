use leptos::prelude::*;

use crate::components::SiteSidebar;

#[component]
pub fn ContentLayout(
    title: &'static str,
    description: &'static str,
    topics: Vec<&'static str>,
) -> impl IntoView {
    // Prepare separate values for the sidebar and main content.
    let show_sidebar = !topics.is_empty();
    let sidebar_topics = topics.clone();
    let content_topics = topics;

    view! {
        <div class="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
            <div class="grid grid-cols-1 gap-8 lg:grid-cols-[16rem_minmax(0,1fr)]">

                // Left-hand sidebar appears only when topics exist.
                <Show when=move || show_sidebar>
                    <SiteSidebar items=sidebar_topics.clone() />
                </Show>

                // Right-hand content.
                <main class="min-w-0">
                    <div class="border-b border-slate-200 pb-6">
                        <h1 class="text-3xl font-bold tracking-tight text-slate-900">{title}</h1>

                        <p class="mt-3 text-lg text-slate-600">{description}</p>
                    </div>

                    <div class="mt-8 space-y-6">
                        {content_topics
                            .into_iter()
                            .enumerate()
                            .map(|(index, topic)| {
                                let section_id = format!("topic-{index}");

                                view! {
                                    <section
                                        id=section_id
                                        class="scroll-mt-28 rounded-xl border border-slate-200 bg-white p-6"
                                    >
                                        <h2 class="text-xl font-semibold text-slate-900">
                                            {topic}
                                        </h2>

                                        <p class="mt-3 text-slate-600">
                                            "Add lessons, explanations, and practice problems here."
                                        </p>
                                    </section>
                                }
                            })
                            .collect_view()}
                    </div>
                </main>
            </div>
        </div>
    }
}
