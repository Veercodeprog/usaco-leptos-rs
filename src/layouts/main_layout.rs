use crate::components::SiteSidebar;
use leptos::prelude::*;

#[component]
pub fn MainLayout(children: Children, sidebar_items: Vec<&'static str>) -> impl IntoView {
    view! {
        <div class="flex w-full min-h-screen">

            // Left navigation
            <aside class="
            hidden lg:block
            w-64 shrink-0
            border-r border-slate-200
            bg-white
            ">
                <div class="
                sticky
                top-0
                h-screen
                overflow-y-auto
                p-4
                ">
                    <SiteSidebar items=sidebar_items />
                </div>
            </aside>

            // Main learning content
            <main class="
            min-w-0 flex-1
            bg-slate-50
            px-6 py-8
            lg:px-10
            ">{children()}</main>

        </div>
    }
}
