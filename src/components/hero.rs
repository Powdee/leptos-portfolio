use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="relative isolate">
            <div class="overflow-hidden">
                <div class="mx-auto max-w-7xl px-6 pb-32 pt-36 sm:pt-60 lg:px-8 lg:pt-72">
                    <div class="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
                        <div class="w-full">
                            <h1 class="text-6xl tracking-tight text-gray-900 sm:text-10xl leading-tighter">
                                I <span class="font-light">develop</span> <br/> digital products
                                <br/> from scratch
                            </h1>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

