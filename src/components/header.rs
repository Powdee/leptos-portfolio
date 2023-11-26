use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="mx-auto max-w-full py-6 px-10 md:py-12 md:px-16">
            <nav
                class="gap-2 md:flex-row flex-col flex items-center justify-between"
                aria-label="Global"
            >
                <h4 class="text-black tracking-wide text-2xl flex lg:flex-1">
                    Hey, "I’m" Erik "👋"
                </h4>
                <div class="items-center justify-end gap-1 flex lg:flex-1">
                    <h4
                        hx-get="link.html"
                        hx-trigger="click"
                        hx-target="#see_more"
                        class="cursor-pointer text-gray text-sm sm:text-md dark:text-gray"
                    >
                        This website was build using Rust
                    </h4>
                    <div
                        id="see_more"
                        class="items-center justify-end"
                        hx-trigger="once"
                        hx-get="link.html"
                    ></div>
                </div>
            </nav>
        </header>
    }
}

