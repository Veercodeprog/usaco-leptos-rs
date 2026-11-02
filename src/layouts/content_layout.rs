use leptos::prelude::*;

#[component]
pub fn ContentLayout(
    title: &'static str,
    description: &'static str,
    topics: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="w-full">

            // Page heading
            <header class="mb-10 border-b border-slate-200 pb-6">
                <h1 class="text-4xl font-bold tracking-tight text-slate-900">{title}</h1>

                <p class="mt-3 text-lg text-slate-600">{description}</p>
            </header>

            // Learning sections
            <div class="w-full space-y-10">
                {topics
                    .into_iter()
                    .enumerate()
                    .map(|(index, topic)| {
                        let section_id = format!("topic-{index}");

                        view! {
                            <section
                                id=section_id
                                class="w-full scroll-mt-8 border-b border-slate-200 pb-10"
                            >
                                <h2 class="text-2xl font-bold text-slate-900">{topic}</h2>

                                <div class="mt-5 rounded-xl border border-slate-200 bg-white p-6">
                                    <p class="text-base leading-7 text-slate-700">
                                        "Add your lesson explanation here."
                                    </p>
                                </div>
                            </section>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
