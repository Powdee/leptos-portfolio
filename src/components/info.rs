use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn Info() -> impl IntoView {
    view! {
        <Layout
            aria_label="Info"
            class_name="bg-gray-9 max-w-full py-28 lg:py-64 rounded-tr-[44px] rounded-tl-[44px] md:rounded-tr-[88px] md:rounded-tl-[88px]"
        >
            <div class="flex-col relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-8xl 2xl:max-w-10xl px-4 md:px-6">
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl tracking-tight text-gray-1 leading-tighter mb-28">
                    Get to know <br/> <span class="font-light">me</span>
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

                <div class="mt-40 lg:mt-64 flex flex-col items-center justify-center">
                    <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight text-gray-1 leading-tighter text-center">
                        <span class="font-light">Want to <br/> chat?</span>
                    </h1>

                    <button
                        type="button"
                        class="w-full md:w-[680px] mt-20 lg:mt-40 rounded-full outline-none font-normal text-lg md:text-3xl px-4 py-8 md:py-12 lg:py-16 text-gray-9 bg-gray-1 shadow-sm hover:bg-gray-9 hover:text-gray-1 ring-1 ring-gray-1 hover:ring-gray-1"
                    >
                        "Let's schedule a call"
                    </button>
                </div>
            </div>
        </Layout>
    }
}

