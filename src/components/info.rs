use crate::components::ui::{
    button::Button, viewport_visiblity::ViewportVisibility,
};

use leptos::{html::Section, *};
use leptos_use::{use_element_visibility, use_window_scroll};

#[component]
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
        </ViewportVisibility>
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
                <a class="text-2xl md:text-3xl lg:text-4xl text-gray-1" href="/resume">
                    Résumé
                </a>
                <div class="w-full h-[2px] bg-gray-1"></div>
            </div>
        </div>
    }
}

#[component]
pub fn InfoAction() -> impl IntoView {
    view! {
        <ViewportVisibility fallback=|| {
            view! { <div class="min-h-[270px] md:min-h-[850px]"></div> }
        }>
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
                <Button
                    label="Let's schedule a call".to_string()
                    class_name="mt-16 lg:mt-32 md:w-[680px] inverse".to_string()
                />
            </div>
        </ViewportVisibility>
    }
}

fn map_y_to_value(y: f64, y_visible_coord: f64) -> f64 {
    let start_y = y_visible_coord;
    let end_y = y_visible_coord + 500.0;
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
    let el = create_node_ref::<Section>();
    let is_element_visible = use_element_visibility(el);
    let (_, y) = use_window_scroll();

    let (y_visible_coord, set_y_visible_coord) = create_signal::<f64>(0.0);

    create_effect(move |_| {
        if is_element_visible.get() && y_visible_coord.get() == 0.0 {
            set_y_visible_coord(y.get());
        }
    });

    view! {
        <section
            node_ref=el
            aria_label="Info"
            class="bg-gray-9 max-w-full py-28 lg:py-64 relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-8xl 2xl:max-w-10xl px-4 md:px-6"
            style=move || {
                format!(
                    "border-top-left-radius: {}px;border-top-right-radius: {}px",
                    map_y_to_value(y.get(), y_visible_coord.get()),
                    map_y_to_value(y.get(), y_visible_coord.get()),
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
