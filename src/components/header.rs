use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="mx-auto max-w-full py-6 px-10 md:py-12 md:px-16 selection:bg-gray-9 selection:text-gray-1">
            <nav
                class="gap-2 md:flex-row flex-col flex items-center justify-between"
                aria-label="Global"
            >
                <h4 class="text-gray-9 tracking-wide text-xl md:text-2xl flex lg:flex-1">
                    Hey, "I’m" Erik Kurjak "👋"
                </h4>
                <div class="items-center justify-end gap-1 flex lg:flex-1">
                    <h4 class="text-gray-7 text-sm sm:text-md">
                        This website was build using Rust
                    </h4>
                </div>
            </nav>
        </header>
    }
}

