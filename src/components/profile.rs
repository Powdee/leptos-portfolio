use crate::components::ui::me_circle::{MeCircle, MeCircleTablet};

use leptos::*;
#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <div class=" w-full h-full lg:w-56 lg:h-56 hidden lg:flex justify-center items-center fixed right-10 bottom-10 z-10">
            <div class="w-48 h-48 rounded-full bg-ek-orange flex justify-center items-center">
                <picture class="absolute flex items-center">
                    <source
                        type="image/webp"
                        srcset="https://leptoscv.s3.eu-central-1.amazonaws.com/me.webp"
                    />
                    <img
                        width="400"
                        height="400"
                        loading="lazy"
                        class="rounded-full object-cover w-[140px] h-[140px] will-change-auto bg-ek-orange"
                        decoding="async"
                        alt="erik kurjak"
                        src="https://leptoscv.s3.eu-central-1.amazonaws.com/me.webp"
                    />
                </picture>
                <MeCircle/>
                <MeCircleTablet/>
            </div>
        </div>
    }
}

