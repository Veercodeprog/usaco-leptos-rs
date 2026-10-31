use leptos::prelude::*;

#[component]
pub fn ContentLayout(
    title: &'static str,
    description: &'static str,
    topics: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="mx-auto max-w-5xl">

            <div class="border-b border-slate-200 pb-6">
                <h1 class="text-3xl font-bold tracking-tight text-slate-900">{title}</h1>

                <p class="mt-3 text-lg text-slate-600">{description}</p>
            </div>

            <div class="mt-8 space-y-6">
                {topics
                    .into_iter()
                    .enumerate()
                    .map(|(index, topic)| {
                        let section_id = format!("topic-{index}");

                        view! {
                            <section
                                id=section_id
                                class="
                                scroll-mt-28
                                rounded-xl
                                border
                                border-slate-200
                                bg-white
                                p-6
                                "
                            >
                                <h2 class="text-xl font-semibold text-slate-900">{topic}</h2>

                                <p class="mt-3 text-slate-600">
                                    "Add lessons, explanations, and practice problems here."
                                </p>
                            </section>
                        }
                    })
                    .collect_view()}
            </div>

        </div>
    }
}
