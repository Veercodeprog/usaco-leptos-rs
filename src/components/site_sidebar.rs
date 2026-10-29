use leptos::prelude::*;

#[component]
pub fn SiteSidebar(items: Vec<&'static str>) -> impl IntoView {
    view! {
        <aside class="hidden lg:block lg:w-64 lg:shrink-0">
            <nav
                aria-label="Page sections"
                class="sticky top-6 rounded-xl border border-slate-200 bg-white p-4"
            >
                <h2 class="mb-3 px-3 text-xs font-bold uppercase tracking-wider text-slate-500">
                    "On this page"
                </h2>

                <ul class="space-y-1">
                    {items
                        .into_iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let href = format!("#topic-{index}");

                            view! {
                                <li>
                                    <a
                                        href=href
                                        class="block rounded-lg border-l-2 border-transparent px-3 py-2 text-sm font-medium text-slate-600 transition hover:border-indigo-500 hover:bg-indigo-50 hover:text-indigo-700 focus-visible:outline-2 focus-visible:outline-indigo-600"
                                    >
                                        {item}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </nav>
        </aside>
    }
}
