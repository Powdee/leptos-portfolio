use crate::components::ui::viewport_visiblity::ViewportVisibility;

use leptos::*;
use leptos_use::use_window_scroll;

#[island]
pub fn InfoSocials() -> impl IntoView {
    view! {
        <ViewportVisibility>
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl tracking-tight leading-tighter mb-28">
                <div class="animated-title">
                    <span class="animated-title-element text-gray-1">Get</span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-gray-1">to</span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-gray-1">know</span>
                </div>
                <br/>
                <div class="animated-title">
                    <em class="animated-title-element font-light text-gray-1">me</em>
                </div>
            </h1>

            <div class="flex flex-col gap-8 md:gap-10">
                <div class="flex flex-col gap-8 md:gap-10">
                    <a
                        target="_blank"
                        class="text-2xl md:text-3xl lg:text-4xl text-gray-1"
                        href="https://github.com/Powdee"
                    >
                        Github
                    </a>
                    <div class="w-full h-[2px] bg-gray-1"></div>
                </div>

                <div class="flex flex-col gap-8 md:gap-10">
                    <a
                        target="_blank"
                        class="text-2xl md:text-3xl lg:text-4xl text-gray-1"
                        href="https://www.instagram.com/erik.kurjak/"
                    >
                        Instagram
                    </a>
                    <div class="w-full h-[2px] bg-gray-1"></div>
                </div>

                <div class="flex flex-col gap-8 md:gap-10">
                    <a
                        target="_blank"
                        class="text-2xl md:text-3xl lg:text-4xl text-gray-1"
                        href="#"
                    >
                        Download CV
                    </a>
                    <div class="w-full h-[2px] bg-gray-1"></div>
                </div>
            </div>
        </ViewportVisibility>
    }
}

#[island]
pub fn InfoAction() -> impl IntoView {
    view! {
        <ViewportVisibility>
            <div class="mt-40 lg:mt-64 flex flex-col items-center justify-center">
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight leading-tighter text-center">
                    <div class="animated-title">
                        <span class="animated-title-element font-light text-gray-1">Want</span>
                    </div>
                    {' '}
                    <div class="animated-title">
                        <span class="animated-title-element font-light text-gray-1">to</span>
                    </div>

                    <br/>
                    <div class="animated-title">
                        <span class="animated-title-element font-light text-gray-1">chat?</span>
                    </div>
                </h1>
                <button
                    type="button"
                    class="w-full md:w-[680px] mt-20 lg:mt-40 rounded-full outline-none font-normal text-lg md:text-3xl px-4 py-8 md:py-12 lg:py-16 text-gray-9 bg-gray-1 shadow-sm hover:bg-gray-9 hover:text-gray-1 ring-1 ring-gray-1 hover:ring-gray-1"
                >
                    "Let's schedule a call"
                </button>
            </div>
        </ViewportVisibility>
    }
}

fn map_y_to_value(y: f64) -> f64 {
    let start_y = 3600.0;
    let end_y = 4300.0;
    let start_value = 120.0;
    let end_value = 0.0;

    if y < start_y {
        return start_value;
    }
    if y > end_y {
        return end_value;
    }

    let scale = (y - start_y) / (end_y - start_y);
    start_value + scale * (end_value - start_value)
}

#[island]
pub fn Info() -> impl IntoView {
    let (_, y) = use_window_scroll();

    view! {
        <section
            aria_label="Info"
            class="bg-gray-9 max-w-full py-28 lg:py-64 relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-8xl 2xl:max-w-10xl px-4 md:px-6"
            style=move || {
                format!(
                    "border-top-left-radius: {}px;border-top-right-radius: {}px",
                    map_y_to_value(y.get()),
                    map_y_to_value(y.get()),
                )
            }
        >

            <div class="flex-col relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-8xl 2xl:max-w-10xl px-4 md:px-6">
                <InfoSocials/>
                <InfoAction/>
            </div>
        </section>
    }
}

