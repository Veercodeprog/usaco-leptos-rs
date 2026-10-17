#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-slate-50 text-slate-900">

            // Introduction
            <section class="border-b border-slate-200 bg-white">
                <div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 sm:py-16 lg:px-8">
                    <p class="text-sm font-semibold uppercase tracking-widest text-indigo-600">
                        "Learn. Practice. Build."
                    </p>

                    <h1 class="mt-4 max-w-3xl text-4xl font-bold tracking-tight sm:text-5xl">
                        "Build strong fundamentals."
                        <span class="block text-indigo-600">"Turn them into real projects."</span>
                    </h1>

                    <p class="mt-5 max-w-2xl text-base leading-7 text-slate-600 sm:text-lg">
                        "Explore data structures and algorithms by topic, then apply
                        what you learn through practical development projects."
                    </p>

                    <div class="mt-8 flex flex-wrap gap-3">
                        <a
                            href="#dsa"
                            class="inline-flex items-center justify-center rounded-lg
                             bg-indigo-600 px-5 py-3 text-sm font-semibold
                             text-white transition hover:bg-indigo-700
                             focus-visible:outline-2 focus-visible:outline-offset-4
                             focus-visible:outline-indigo-600"
                        >
                            "Explore DSA"
                            <span aria-hidden="true" class="ml-2">
                                "→"
                            </span>
                        </a>

                        <a
                            href="#projects"
                            class="inline-flex items-center justify-center rounded-lg
                             border border-slate-300 bg-white px-5 py-3
                             text-sm font-semibold text-slate-700 transition
                             hover:border-slate-400 hover:bg-slate-50
                             focus-visible:outline-2 focus-visible:outline-offset-4
                             focus-visible:outline-indigo-600"
                        >
                            "Browse Projects"
                        </a>
                    </div>
                </div>
            </section>

            <div class="mx-auto max-w-7xl space-y-16 px-4 py-12 sm:px-6 lg:px-8">

                // DSA section
                <section id="dsa" aria-labelledby="dsa-heading" class="scroll-mt-24">
                    <div class="mb-6">
                        <p class="text-sm font-semibold text-indigo-600">"Practice by topic"</p>

                        <h2
                            id="dsa-heading"
                            class="mt-2 text-2xl font-bold tracking-tight sm:text-3xl"
                        >
                            "Data Structures & Algorithms"
                        </h2>

                        <p class="mt-3 max-w-2xl leading-7 text-slate-600">
                            "Start with the fundamentals and work your way toward
                            more advanced problem-solving patterns."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
                        <HomeCategoryCard
                            href="/dsa/arrays-and-strings"
                            label="01"
                            title="Arrays & Strings"
                            description="Practice traversal, prefix sums, two pointers,
                             sliding windows, and string manipulation."
                            tags="Arrays · Strings · Two Pointers"
                            action="Explore problems"
                        />

                        <HomeCategoryCard
                            href="/dsa/linked-lists"
                            label="02"
                            title="Linked Lists"
                            description="Understand pointer manipulation through
                             reversals, cycle detection, and list merging."
                            tags="Pointers · Fast & Slow · Reversal"
                            action="Explore problems"
                        />

                        <HomeCategoryCard
                            href="/dsa/stacks-and-queues"
                            label="03"
                            title="Stacks & Queues"
                            description="Work through expression parsing, monotonic
                             stacks, deques, and processing sequences."
                            tags="Stacks · Queues · Monotonic Patterns"
                            action="Explore problems"
                        />

                        <HomeCategoryCard
                            href="/dsa/trees-and-graphs"
                            label="04"
                            title="Trees & Graphs"
                            description="Explore connected structures with traversals,
                             search, shortest paths, and tree algorithms."
                            tags="DFS · BFS · Shortest Paths"
                            action="Explore problems"
                        />

                        <HomeCategoryCard
                            href="/dsa/dynamic-programming"
                            label="05"
                            title="Dynamic Programming"
                            description="Break problems into reusable subproblems
                             using memoization and tabulation."
                            tags="States · Transitions · Optimization"
                            action="Explore problems"
                        />

                        <HomeCategoryCard
                            href="/dsa/searching-and-sorting"
                            label="06"
                            title="Searching & Sorting"
                            description="Practice binary search, ordering techniques,
                             and divide-and-conquer approaches."
                            tags="Binary Search · Sorting · Divide & Conquer"
                            action="Explore problems"
                        />
                    </div>
                </section>

                // Projects section
                <section id="projects" aria-labelledby="projects-heading" class="scroll-mt-24">
                    <div class="mb-6">
                        <p class="text-sm font-semibold text-indigo-600">"Learn by building"</p>

                        <h2
                            id="projects-heading"
                            class="mt-2 text-2xl font-bold tracking-tight sm:text-3xl"
                        >
                            "Projects"
                        </h2>

                        <p class="mt-3 max-w-2xl leading-7 text-slate-600">
                            "Choose a project track and put your skills to work
                            building something beyond individual coding problems."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
                        <HomeCategoryCard
                            href="/projects/frontend"
                            label="UI"
                            title="Frontend Development"
                            description="Build responsive interfaces, interactive
                             dashboards, and reusable UI components."
                            tags="Layouts · Components · Interactivity"
                            action="Browse projects"
                        />

                        <HomeCategoryCard
                            href="/projects/backend"
                            label="API"
                            title="Backend Development"
                            description="Create APIs and services with authentication,
                             database storage, and background processing."
                            tags="APIs · Databases · Services"
                            action="Browse projects"
                        />

                        <HomeCategoryCard
                            href="/projects/full-stack"
                            label="WEB"
                            title="Full-Stack Applications"
                            description="Connect frontend and backend systems to build
                             complete applications from end to end."
                            tags="Integration · Auth · Deployment"
                            action="Browse projects"
                        />

                        <HomeCategoryCard
                            href="/projects/rust"
                            label="RS"
                            title="Rust & Systems"
                            description="Explore ownership and concurrency through
                             command-line tools and systems projects."
                            tags="Rust · CLI · Concurrency"
                            action="Browse projects"
                        />

                        <HomeCategoryCard
                            href="/projects/data"
                            label="DATA"
                            title="Data & Visualization"
                            description="Transform datasets into useful insights with
                             processing pipelines and visual dashboards."
                            tags="Processing · Analytics · Charts"
                            action="Browse projects"
                        />

                        <HomeCategoryCard
                            href="/projects/developer-tools"
                            label="DEV"
                            title="Developer Tools"
                            description="Build tools that automate repetitive work
                             and improve everyday development workflows."
                            tags="Automation · Tooling · Productivity"
                            action="Browse projects"
                        />
                    </div>
                </section>

            </div>
        </div>
    }
}

#[component]
fn HomeCategoryCard(
    href: &'static str,
    label: &'static str,
    title: &'static str,
    description: &'static str,
    tags: &'static str,
    action: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=href
            class="group flex h-full flex-col rounded-xl border border-slate-200
             bg-white p-6 transition
             hover:-translate-y-1 hover:border-indigo-300 hover:shadow-md
             motion-reduce:transform-none motion-reduce:transition-none
             focus-visible:outline-2 focus-visible:outline-offset-4
             focus-visible:outline-indigo-600"
        >
            <div class="flex items-center justify-between gap-4">
                <span
                    aria-hidden="true"
                    class="inline-flex h-12 min-w-12 items-center justify-center
                     rounded-lg bg-indigo-50 px-2 font-mono text-sm
                     font-bold text-indigo-600"
                >
                    {label}
                </span>

                <span
                    aria-hidden="true"
                    class="text-xl text-slate-400 transition
                     group-hover:text-indigo-600"
                >
                    "↗"
                </span>
            </div>

            <h3 class="mt-5 text-lg font-semibold tracking-tight text-slate-900">{title}</h3>

            <p class="mt-2 text-sm leading-6 text-slate-600">{description}</p>

            <p class="mt-4 text-xs font-medium leading-5 text-slate-500">{tags}</p>

            <div class="mt-auto pt-6">
                <span class="text-sm font-semibold text-indigo-600">
                    {action} <span aria-hidden="true" class="ml-1">
                        "→"
                    </span>
                </span>
            </div>
        </a>
    }
}
