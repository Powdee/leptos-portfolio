use leptos::*;

#[component]
pub fn Me() -> impl IntoView {
    view! {
        <div class="relative isolate">
            <div class="overflow-hidden">
                <div class="mx-auto max-w-7xl px-6 pb-32 pt-36 sm:pt-60 lg:px-8 lg:pt-60">
                    <div class="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
                        <div class="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
                            <div class="w-full max-w-xl lg:shrink-0 xl:max-w-2xl">
                                <img
                                    class="w-96 h-96 bg-cover bg-no-repeat bg-center rounded-full"
                                    style="background-image: url(assets/me.jpg)"
                                    title="me"
                                    alt="erik kurjak"
                                />
                                Open to new challenges
                            </div>
                            <div class="mt-14 flex flex-col justify-end gap-8 sm:-mt-44 sm:justify-start sm:pl-20 lg:mt-0 lg:pl-0">
                                <p class="relative mt-0 mb-6 text-2xl leading-relaxish text-black sm:max-w-md lg:max-w-none">
                                    A team player with a passion for building modern digital products. With a strong affinity for functional programming and a natural problem-solving ability.
                                </p>
                                <button
                                    type="button"
                                    class="rounded-full font-normal text-2xl bg-white px-4 py-16 text-black shadow-sm ring-1 ring-inset ring-gray hover:bg-black hover:text-white"
                                >
                                    "Let's connect"
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

