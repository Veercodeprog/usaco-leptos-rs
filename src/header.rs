use leptos::prelude::*;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let mobile_open = RwSignal::new(false);

    view! {
        <header class="relative z-50">

            // =========================================================
            // PROMO BAR
            // =========================================================
            <div class="border-b border-gray-200 bg-gradient-to-r from-pink-50 via-white to-purple-50">
                <div class="mx-auto flex h-20 max-w-7xl items-center justify-center px-4">
                    <div class="flex items-center gap-6 text-lg text-gray-900">
                        <span>"Our fall semester of Bronze/Silver live classes starts soon."</span>

                        <span class="text-gray-700">"•"</span>

                        <a
                            href="https://joincpi.org/classes"
                            target="_blank"
                            rel="noreferrer"
                            class="
                            rounded-full
                            bg-gray-900
                            px-6
                            py-2
                            font-semibold
                            text-white
                            shadow-sm
                            transition
                            hover:bg-gray-800
                            "
                        >
                            "Register here →"
                        </a>
                    </div>
                </div>
            </div>

            // =========================================================
            // MAIN NAVBAR
            // =========================================================
            <nav class="border-b border-gray-200 bg-white shadow-sm">

                <div class="mx-auto max-w-7xl px-2 sm:px-4 lg:px-8">

                    // =================================================
                    // ONE HORIZONTAL ROW
                    // =================================================
                    <div class="flex h-20 flex-row items-center justify-between">

                        // =================================================
                        // LEFT SIDE
                        // =================================================
                        <div class="flex h-full flex-row items-center">

                            // ------------------------------------------------
                            // LOGO
                            // ------------------------------------------------
                            <a href="/" class="flex h-full shrink-0 flex-row items-center">
                                <div class="flex flex-row items-center whitespace-nowrap">

                                    <svg
                                        width="56"
                                        height="56"
                                        viewBox="0 0 100 100"
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="block h-14 w-14 shrink-0"
                                    >
                                        <path
                                            d="M50,5A45,45,0,1,1,5,50,45.05,45.05,0,0,1,50,5m0-5a50,50,0,1,0,50,50A50,50,0,0,0,50,0Z"
                                            fill="#143f9f"
                                        />

                                        <line
                                            x1="50"
                                            y1="27"
                                            x2="73.29"
                                            y2="65.64"
                                            stroke="#143f9f"
                                            stroke-width="7"
                                        />

                                        <line
                                            x1="50"
                                            y1="27"
                                            x2="28.56"
                                            y2="67"
                                            stroke="#143f9f"
                                            stroke-width="7"
                                        />

                                        <circle cx="50" cy="27" r="10" fill="#143f9f" />

                                        <circle cx="28.56" cy="67" r="10" fill="#143f9f" />

                                        <circle cx="73.29" cy="65.64" r="10" fill="#143f9f" />
                                    </svg>

                                    <span class="ml-3 text-3xl font-bold tracking-tight text-black">
                                        "USACO Guide"
                                    </span>

                                </div>
                            </a>

                            // =================================================
                            // DESKTOP MENU
                            // =================================================
                            <div class="
                            ml-12
                            hidden
                            h-full
                            flex-row
                            items-center
                            lg:flex
                            lg:space-x-10
                            ">

                                // =================================================
                                // SECTIONS
                                // =================================================
                                <div class="group relative flex h-full flex-row items-center">

                                    <button
                                        type="button"
                                        class="
                                        inline-flex
                                        h-full
                                        flex-row
                                        items-center
                                        gap-2
                                        border-b-2
                                        border-transparent
                                        px-1
                                        text-xl
                                        font-medium
                                        text-gray-500
                                        transition
                                        hover:border-gray-300
                                        hover:text-gray-900
                                        "
                                    >
                                        <span>"Sections"</span>

                                        <svg
                                            width="22"
                                            height="22"
                                            viewBox="0 0 20 20"
                                            fill="currentColor"
                                            class="block h-[22px] w-[22px] shrink-0 text-gray-400"
                                        >
                                            <path
                                                fill-rule="evenodd"
                                                d="M5.23 7.21a.75.75 0 011.06.02L10 11.17l3.71-3.94a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0L5.21 8.27a.75.75 0 01.02-1.06z"
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                    </button>

                                    <div class="
                                    invisible
                                    absolute
                                    left-0
                                    top-full
                                    z-50
                                    w-56
                                    rounded-lg
                                    bg-white
                                    py-2
                                    opacity-0
                                    shadow-lg
                                    ring-1
                                    ring-black/5
                                    transition
                                    duration-150
                                    group-hover:visible
                                    group-hover:opacity-100
                                    ">

                                        <a
                                            href="/general"
                                            class="block px-4 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                                        >
                                            "General"
                                        </a>

                                        <a
                                            href="/bronze"
                                            class="block px-4 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                                        >
                                            "Bronze"
                                        </a>

                                        <a
                                            href="/silver"
                                            class="block px-4 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                                        >
                                            "Silver"
                                        </a>

                                        <a
                                            href="/gold"
                                            class="block px-4 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                                        >
                                            "Gold"
                                        </a>

                                        <a
                                            href="/plat"
                                            class="block px-4 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                                        >
                                            "Platinum"
                                        </a>

                                        <a
                                            href="/adv"
                                            class="block px-4 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                                        >
                                            "Advanced"
                                        </a>

                                    </div>
                                </div>

                                // =================================================
                                // PROBLEMS
                                // =================================================
                                <a
                                    href="/problems"
                                    class="
                                    inline-flex
                                    h-full
                                    flex-row
                                    items-center
                                    border-b-2
                                    border-transparent
                                    px-1
                                    text-xl
                                    font-medium
                                    text-gray-500
                                    transition
                                    hover:border-gray-300
                                    hover:text-gray-900
                                    "
                                >
                                    "Problems"
                                </a>

                                // =================================================
                                // RESOURCES
                                // =================================================
                                <div class="group relative flex h-full flex-row items-center">

                                    <button
                                        type="button"
                                        class="
                                        inline-flex
                                        h-full
                                        flex-row
                                        items-center
                                        gap-2
                                        border-b-2
                                        border-transparent
                                        px-1
                                        text-xl
                                        font-medium
                                        text-gray-500
                                        transition
                                        hover:border-gray-300
                                        hover:text-gray-900
                                        "
                                    >
                                        <span>"Resources"</span>

                                        <svg
                                            width="22"
                                            height="22"
                                            viewBox="0 0 20 20"
                                            fill="currentColor"
                                            class="block h-[22px] w-[22px] shrink-0 text-gray-400"
                                        >
                                            <path
                                                fill-rule="evenodd"
                                                d="M5.23 7.21a.75.75 0 011.06.02L10 11.17l3.71-3.94a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0L5.21 8.27a.75.75 0 01.02-1.06z"
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                    </button>

                                    <div class="
                                    invisible
                                    absolute
                                    left-1/2
                                    top-full
                                    z-50
                                    w-[560px]
                                    -translate-x-1/2
                                    rounded-lg
                                    bg-white
                                    p-8
                                    opacity-0
                                    shadow-xl
                                    ring-1
                                    ring-black/5
                                    transition
                                    duration-150
                                    group-hover:visible
                                    group-hover:opacity-100
                                    ">

                                        <div class="grid grid-cols-2 gap-6">

                                            <a href="/groups" class="rounded-lg p-3 hover:bg-gray-100">
                                                <p class="text-base font-medium text-gray-900">"Groups"</p>

                                                <p class="mt-1 text-sm text-gray-500">
                                                    "A learning management system fully integrated with the USACO Guide."
                                                </p>
                                            </a>

                                            <a href="/editor" class="rounded-lg p-3 hover:bg-gray-100">
                                                <p class="text-base font-medium text-gray-900">"Editor"</p>

                                                <p class="mt-1 text-sm text-gray-500">
                                                    "An online editor for writing content."
                                                </p>
                                            </a>

                                            <a href="#" class="rounded-lg p-3 hover:bg-gray-100">
                                                <p class="text-base font-medium text-gray-900">
                                                    "Graph Visualizer"
                                                </p>

                                                <p class="mt-1 text-sm text-gray-500">
                                                    "A tool for visualizing graphs and algorithms."
                                                </p>
                                            </a>

                                            <a
                                                href="https://forum.usaco.guide/"
                                                target="_blank"
                                                rel="noreferrer"
                                                class="rounded-lg p-3 hover:bg-gray-100"
                                            >
                                                <p class="text-base font-medium text-gray-900">
                                                    "USACO Forum"
                                                </p>

                                                <p class="mt-1 text-sm text-gray-500">
                                                    "An unofficial Q&A forum for USACO contestants."
                                                </p>
                                            </a>

                                        </div>
                                    </div>
                                </div>

                                // =================================================
                                // CONTACT US
                                // =================================================
                                <a
                                    href="/contact"
                                    class="
                                    inline-flex
                                    h-full
                                    flex-row
                                    items-center
                                    border-b-2
                                    border-transparent
                                    px-1
                                    text-xl
                                    font-medium
                                    text-gray-500
                                    transition
                                    hover:border-gray-300
                                    hover:text-gray-900
                                    "
                                >
                                    "Contact Us"
                                </a>

                            </div>
                        </div>

                        // =================================================
                        // RIGHT SIDE
                        // =================================================
                        <div class="flex flex-row items-center justify-end">

                            // Search
                            <button
                                type="button"
                                class="
                                inline-flex
                                flex-row
                                items-center
                                gap-2
                                px-3
                                py-2
                                text-xl
                                text-gray-500
                                transition
                                hover:text-gray-700
                                "
                            >
                                <svg
                                    width="25"
                                    height="25"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    class="block h-[25px] w-[25px] shrink-0 text-gray-400"
                                >
                                    <circle cx="11" cy="11" r="7" />
                                    <path d="M20 20l-4-4" />
                                </svg>

                                <span>"Search"</span>
                            </button>

                            // Divider
                            <div class="mx-4 h-10 border-l border-gray-300"></div>

                            // User avatar
                            <button
                                type="button"
                                class="
                                flex
                                h-12
                                w-12
                                items-center
                                justify-center
                                rounded-full
                                bg-[#415967]
                                text-xl
                                font-medium
                                text-white
                                "
                            >
                                "V"
                            </button>

                            // Mobile button
                            <button
                                type="button"
                                class="ml-3 block p-2 lg:hidden"
                                on:click=move |_| {
                                    mobile_open.update(|v| *v = !*v);
                                }
                            >
                                <svg
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    class="block h-6 w-6"
                                >
                                    <path stroke-linecap="round" d="M4 6h16M4 12h16M4 18h16" />
                                </svg>
                            </button>

                        </div>
                    </div>
                </div>

                // =========================================================
                // MOBILE MENU
                // =========================================================
                <Show when=move || mobile_open.get()>
                    <div class="border-t border-gray-200 bg-white lg:hidden">

                        <div class="space-y-1 px-4 py-5">

                            <a
                                href="/general"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "General"
                            </a>

                            <a
                                href="/bronze"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Bronze"
                            </a>

                            <a
                                href="/silver"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Silver"
                            </a>

                            <a
                                href="/gold"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Gold"
                            </a>

                            <a
                                href="/plat"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Platinum"
                            </a>

                            <a
                                href="/adv"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Advanced"
                            </a>

                            <a
                                href="/problems"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Problems"
                            </a>

                            <a
                                href="/contact"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Contact Us"
                            </a>

                            <a
                                href="/settings"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Settings"
                            </a>

                            <a
                                href="/login"
                                class="block rounded-md px-3 py-2 text-base font-medium text-gray-700 hover:bg-gray-100"
                            >
                                "Sign In"
                            </a>

                        </div>
                    </div>
                </Show>

            </nav>
        </header>
    }
}
