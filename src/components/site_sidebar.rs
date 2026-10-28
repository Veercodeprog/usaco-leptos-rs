use leptos::prelude::*;

#[component]
pub fn SiteSidebar() -> impl IntoView {
    view! {
        <aside aria-label="Page navigation" class="hidden lg:block lg:w-64 lg:shrink-0">
            <nav
                aria-label="On this page"
                class="sticky top-24 rounded-xl border border-slate-200
                bg-white p-4 shadow-sm"
            >
                <h2 class="mb-3 px-3 text-xs font-bold uppercase
                tracking-wider text-slate-500">"On this page"</h2>

                <ul class="space-y-1">
                    <li>
                        <a
                            href="#dsa"
                            class="group flex items-center rounded-lg
                            border-l-2 border-transparent px-3 py-2
                            text-sm font-medium text-slate-600
                            transition hover:border-indigo-500
                            hover:bg-indigo-50 hover:text-indigo-700
                            focus-visible:outline-2
                            focus-visible:outline-indigo-600"
                        >
                            "Data Structures & Algorithms"
                        </a>
                    </li>

                    <li>
                        <a
                            href="#projects"
                            class="group flex items-center rounded-lg
                            border-l-2 border-transparent px-3 py-2
                            text-sm font-medium text-slate-600
                            transition hover:border-indigo-500
                            hover:bg-indigo-50 hover:text-indigo-700
                            focus-visible:outline-2
                            focus-visible:outline-indigo-600"
                        >
                            "Projects"
                        </a>
                    </li>
                </ul>
            </nav>
        </aside>
    }
}
