use leptos::prelude::*;

use crate::components::SiteSidebar;

#[component]
pub fn MainLayout(children: Children, sidebar_items: Vec<&'static str>) -> impl IntoView {
    let items = sidebar_items.clone();

    view! {
        <div class="mx-auto flex max-w-[1600px] min-h-screen">

            // LEFT SIDEBAR
            <aside class="
            hidden
            lg:block
            lg:w-64
            lg:shrink-0
            border-r
            border-slate-200
            bg-white
            ">
                <div class="
                sticky
                top-0
                h-[calc(100vh-5rem)]
                overflow-y-auto
                p-4
                ">
                    <SiteSidebar items=items />
                </div>
            </aside>

            // MAIN CONTENT
            <main class="
            min-w-0
            flex-1
            bg-slate-50
            px-4
            py-8
            sm:px-6
            lg:px-10
            ">{children()}</main>

        </div>
    }
}
