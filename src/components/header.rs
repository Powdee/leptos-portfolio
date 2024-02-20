use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            id="header"
            class="mx-auto max-w-full py-6 px-10 md:py-12 md:px-16 selection:bg-gray-9 selection:text-gray-1 backdrop-invert-0 bg-transparent"
        >
            <nav class="w-full" aria-label="Global">
                <ul class="gap-4 md:flex-row flex-col flex items-center justify-between">
                    <li class="text-gray-9 tracking-wide text-xl md:text-2xl flex lg:flex-1">
                        Hey, "I’m" Erik Kurjak "👋"
                    </li>
                    <li class="hidden md:flex gap-12 items-center">
                        <a
                            href="#home"
                            onclick="lenis.scrollTo('#header')"
                            class="text-gray-9 text-sm sm:text-lg link"
                        >
                            Home
                        </a>
                        <a
                            href="#projects"
                            onclick="lenis.scrollTo('#projects')"
                            class="text-gray-9 text-sm sm:text-lg link"
                        >
                            Projects
                        </a>
                        <a
                            href="#info"
                            onclick="lenis.scrollTo('#contact')"
                            class="text-gray-9 text-sm sm:text-lg link"
                        >
                            Contact
                        </a>
                        <a href="/resume" class="text-gray-9 text-sm sm:text-lg link">
                            Résumé
                        </a>
                    </li>
                // <li class="items-center justify-end gap-1 flex lg:flex-1">
                // <span class="text-gray-7 text-sm sm:text-md">
                // This website was build using Rust
                // </span>
                // </li>
                </ul>
            </nav>
        </header>
    }
}

