use leptos::prelude::*;

use crate::strapi::get_categories;

#[component]
pub fn CategoriesPage() -> impl IntoView {
    let (page, set_page) = signal(1_u32);
    let (pending, set_pending) = signal(false);
    // Whenever `page` changes, fetch the corresponding CMS page.
    let categories = Resource::new(
        move || page.get(),
        |current_page| get_categories(current_page),
    );

    view! {
        <section class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
            <div class="border-b border-slate-200 pb-6">
                <h1 class="text-3xl font-bold tracking-tight text-slate-900">"Categories"</h1>

                <p class="mt-3 text-lg text-slate-600">"Learning categories loaded from Strapi."</p>
            </div>

            <Transition
                set_pending=set_pending
                fallback=move || {
                    view! {
                        <p class="mt-8 text-slate-500" role="status">
                            "Loading categories..."
                        </p>
                    }
                }
            >
                {move || {
                    categories
                        .get()
                        .map(|result| {
                            match result {
                                Err(error) => {
                                    view! {
                                        <div
                                            class="mt-8 rounded-lg border border-red-200 bg-red-50 p-4"
                                            role="alert"
                                        >
                                            <p class="text-red-700">{error.to_string()}</p>

                                            <button
                                                type="button"
                                                class="mt-3 rounded bg-red-700 px-4 py-2 text-white"
                                                on:click=move |_| categories.refetch()
                                            >
                                                "Try again"
                                            </button>
                                        </div>
                                    }
                                        .into_any()
                                }
                                Ok(result) => {
                                    let pagination = result.pagination;
                                    let items = result.categories;
                                    let current_page = pagination.page;
                                    let page_count = pagination.page_count;
                                    let total = pagination.total;
                                    let is_empty = items.is_empty();

                                    view! {
                                        <div class="mt-8">
                                            <p class="mb-4 text-sm text-slate-500">
                                                {format!("Total categories: {total}")}
                                            </p>

                                            {if is_empty {
                                                view! {
                                                    <p class="rounded-lg border border-slate-200 bg-white p-6 text-slate-600">
                                                        "No categories found on this page."
                                                    </p>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                                                        <For
                                                            each=move || items.clone()
                                                            key=|category| category.document_id.clone()
                                                            children=move |category| {
                                                                let description = category
                                                                    .description
                                                                    .filter(|text| !text.trim().is_empty())
                                                                    .unwrap_or_else(|| {
                                                                        "No description available.".to_string()
                                                                    });

                                                                view! {
                                                                    <article class="rounded-xl border border-slate-200 bg-white p-6 shadow-sm">
                                                                        <h2 class="text-xl font-semibold text-slate-900">
                                                                            {category.name}
                                                                        </h2>

                                                                        <p class="mt-2 text-sm text-slate-500">{category.slug}</p>

                                                                        <p class="mt-4 text-slate-600">{description}</p>
                                                                    </article>
                                                                }
                                                            }
                                                        />
                                                    </div>
                                                }
                                                    .into_any()
                                            }}

                                            <div class="mt-8 flex flex-wrap items-center gap-4">
                                                <button
                                                    type="button"
                                                    class="rounded-lg border border-slate-300 bg-white px-4 py-2 disabled:cursor-not-allowed disabled:opacity-50"
                                                    disabled=move || { current_page <= 1 || pending.get() }
                                                    on:click=move |_| {
                                                        set_page
                                                            .update(|value| {
                                                                *value = value.saturating_sub(1).max(1);
                                                            });
                                                    }
                                                >
                                                    "Previous"
                                                </button>

                                                <span class="text-sm text-slate-600">
                                                    {format!("Page {} of {}", current_page, page_count.max(1))}
                                                </span>

                                                <button
                                                    type="button"
                                                    class="rounded-lg border border-slate-300 bg-white px-4 py-2 disabled:cursor-not-allowed disabled:opacity-50"
                                                    disabled=move || {
                                                        current_page >= page_count || pending.get()
                                                    }
                                                    on:click=move |_| {
                                                        set_page
                                                            .update(|value| {
                                                                *value += 1;
                                                            });
                                                    }
                                                >
                                                    "Next"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Transition>
        </section>
    }
}
