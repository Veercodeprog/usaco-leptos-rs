use leptos::prelude::*;

#[component]
pub fn SiteHeader() -> impl IntoView {
    view! {
        <header class="border-b border-[#c8d0da] bg-[#e8edf3] text-[#163a63]">
            <div class="mx-auto max-w-5xl px-4">
                <div class="flex flex-wrap items-center justify-between gap-3 py-5">
                    <a href="/" class="text-2xl font-bold tracking-tight no-underline sm:text-3xl">
                        "USACO Clone"
                    </a>

                    <div class="flex items-center gap-4 text-sm">
                        <a href="#" class="hover:underline">
                            "Login"
                        </a>
                        <a href="#" class="hover:underline">
                            "Register"
                        </a>
                    </div>
                </div>
            </div>

            <nav aria-label="Main navigation" class="bg-[#163a63] text-white">
                <div class="mx-auto flex max-w-5xl flex-wrap px-2">
                    <a
                        href="/"
                        aria-current="page"
                        class="px-3 py-3 text-sm font-semibold hover:bg-[#244f7e]"
                    >
                        "Home"
                    </a>
                    <a href="#" class="px-3 py-3 text-sm font-semibold hover:bg-[#244f7e]">
                        "About"
                    </a>
                    <a href="#" class="px-3 py-3 text-sm font-semibold hover:bg-[#244f7e]">
                        "Contests"
                    </a>
                    <a href="#" class="px-3 py-3 text-sm font-semibold hover:bg-[#244f7e]">
                        "Training"
                    </a>
                    <a href="#" class="px-3 py-3 text-sm font-semibold hover:bg-[#244f7e]">
                        "Resources"
                    </a>
                </div>
            </nav>
        </header>
    }
}
