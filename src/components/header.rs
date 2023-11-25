use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="absolute inset-x-0 top-0 z-50">
            <nav
                class="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8"
                aria-label="Global"
            >
                <h4 class="text-black text-md sm:text-xl flex lg:flex-1">
                    "Hey, I’m Erik Kurjak 👋"
                </h4>
                <div class="items-center justify-end gap-1 flex lg:flex-1">
                    <h4
                        hx-get="link.html"
                        hx-trigger="click"
                        hx-target="#see_more"
                        class="cursor-pointer text-gray text-sm sm:text-md dark:text-gray"
                    >
                        This website was build using Rust and HTMX
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
