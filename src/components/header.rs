use crate::components::ui::assets::globe::Globe;
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Prague;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let utc_now: DateTime<Utc> = Utc::now();
    let prague_time = utc_now.with_timezone(&Prague);
    let time_str = prague_time.format("%H:%M %p").to_string();

    view! {
        <header
            id="header"
            class="sticky top-0 bg-ek-dark mx-auto max-w-full px-3 md:px-5 selection:bg-ek-white selection:text-ek-dark relative backdrop-invert-0 z-10"
        >
            <nav
                class="w-full py-2 md:py-3 border-b-ek-white/[0.35] border-b-2"
                aria-label="Global"
            >
                <ul class="gap-4 flex-row flex items-center justify-between">
                    <li class="font-almarena text-ek-white text-md md:text-lg flex uppercase">
                        Kurjak
                    </li>
                    <li class="hidden lg:flex text-ek-white text-xl md:text-2xl">
                        <span class="text-sm sm:text-md flex items-center gap-2">
                            <svg
                                width="8"
                                height="8"
                                viewBox="0 0 8 8"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <circle cx="4" cy="4" r="4" fill="#00966E"></circle>
                            </svg>
                            Engineering the peer-to-peer betting revolution at Splash
                        </span>
                    </li>
                    <li class="text-ek-white text-xl md:text-2xl flex gap-2 items-center">
                        <Globe/>
                        <span class="text-sm sm:text-md uppercase">Prague, {time_str}</span>
                    </li>
                    <li class="hidden md:flex gap-4 md:gap-8 items-center">
                        <a
                            href="#home"
                            onclick="lenis.scrollTo('#header')"
                            class="text-ek-white text-sm sm:text-md link uppercase"
                        >
                            Home
                        </a>
                        <a
                            href="#projects"
                            onclick="lenis.scrollTo('#projects')"
                            class="text-ek-white text-sm sm:text-md link uppercase"
                        >
                            Projects
                        </a>
                        <a
                            href="#info"
                            onclick="lenis.scrollTo('#contact')"
                            class="text-ek-white text-sm sm:text-md link uppercase"
                        >
                            Contact
                        </a>
                        <a href="/resume" class="text-ek-white text-sm sm:text-md link uppercase">
                            Résumé
                        </a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}

