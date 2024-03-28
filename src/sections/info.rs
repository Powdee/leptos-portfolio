use crate::{
    components::ui::button::Button, utils::map_y_to_value::map_y_to_value,
};

use leptos::{html::Footer, *};
use leptos_use::{use_element_visibility, use_window_scroll};

#[component]
pub fn InfoSocials() -> impl IntoView {
    view! {
        <h2
            class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl mb-28 leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading"
            id="socials"
        >
            <div class="animated-title">
                <span class="animated-title-element text-ek-dark font-bold uppercase">Get</span>
            </div>
            {' '}
            <div class="animated-title">
                <span class="animated-title-element text-ek-dark font-bold uppercase">to</span>
            </div>
            {' '}
            <br/>
            <div class="animated-title">
                <span class="animated-title-element text-ek-dark font-bold uppercase">know</span>
            </div>
            {' '}
            <div class="animated-title">
                <span class="animated-title-element font-light text-ek-dark font-regular uppercase">
                    me
                </span>
            </div>
        </h2>
        <div class="flex flex-col">
            <a
                target="_blank"
                class="text-2xl md:text-3xl lg:text-4xl text-ek-dark font-[400] py-8 md:py-10"
                href="https://github.com/Powdee"
            >
                github
            </a>
            <div class="w-full h-[2px] bg-ek-dark"></div>
            <a
                target="_blank"
                class="text-2xl md:text-3xl lg:text-4xl text-ek-dark font-[400] py-8 md:py-10"
                href="https://www.linkedin.com/in/erik-kurjak/"
            >
                linkedIn
            </a>
            <div class="w-full h-[2px] bg-ek-dark"></div>
            <a
                class="text-2xl md:text-3xl lg:text-4xl text-ek-dark font-[400] py-8 md:py-10"
                href="/resume"
            >
                résumé
            </a>
            <div class="w-full h-[2px] bg-ek-dark"></div>
        </div>
    }
}

#[component]
pub fn InfoAction() -> impl IntoView {
    view! {
        <div class="mt-20 md:mt-40 lg:mt-64 flex flex-col items-center justify-center" id="contact">
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl text-center leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading">
                <div class="animated-title">
                    <span class="animated-title-element text-ek-dark font-bold uppercase">
                        Want
                    </span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-ek-dark font-bold uppercase">to</span>
                </div>

                <br/>
                <div class="animated-title">
                    <span class="animated-title-element text-ek-dark font-bold uppercase">
                        chat?
                    </span>
                </div>
            </h1>
            <Button
                href="mailto:contact@erikkurjak.com".to_string()
                label="Let's schedule a call".to_string()
                class_name="mt-10 md:mt-16 lg:mt-32 md:w-[680px] inverse".to_string()
            />
        </div>
    }
}

#[component]
pub fn Info() -> impl IntoView {
    let el = create_node_ref::<Footer>();
    let is_element_visible = use_element_visibility(el);
    let (_, y) = use_window_scroll();

    let (y_visible_coord, set_y_visible_coord) = create_signal::<f64>(0.0);

    create_effect(move |_| {
        if is_element_visible.get() && y_visible_coord.get() == 0.0 {
            set_y_visible_coord(y.get());
        }
    });

    view! {
        <footer
            node_ref=el
            aria_label="Info"
            id="footer".to_string()
            class="selection:bg-ek-dark selection:text-ek-white bg-ek-white max-w-full pt-28 pb-4 md:py-28 lg:pt-64 lg:pb-32 relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-auto 2xl:max-w-10xl px-4 md:px-6"
            style=move || {
                format!(
                    "border-top-left-radius: {}px;border-top-right-radius: {}px",
                    map_y_to_value(y.get(), y_visible_coord.get() + 140.0),
                    map_y_to_value(y.get(), y_visible_coord.get() + 140.0),
                )
            }
        >

            <div class="flex-col relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-auto 2xl:max-w-10xl px-4 md:px-6">
                <InfoSocials/>
                <InfoAction/>
            </div>
            <span class="md:text-left text-center flex justify-center mt-20 md:absolute md:left-5 md:bottom-5 text-sm sm:text-md text-ek-dark flex items-center">
                "© Copyright 2024, Erik Kurjak, All rights reserved"
            </span>
        </footer>
    }
}

