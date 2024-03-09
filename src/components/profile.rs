use leptos::*;

#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <div class="fade-in w-full h-full lg:w-56 lg:h-56 hidden lg:flex justify-center items-center fixed right-10 bottom-10 z-10">
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
                <svg
                    viewBox="0 0 160 160"
                    width="160"
                    height="160"
                    class="overflow-visible hidden lg:block"
                >
                    <defs>
                        <path
                            id="textPath"
                            d="M 80, 80
                            m -80, 0
                            a 80, 80 0 1,1 160,0
                            a 80,80 0 1,1 -160, 0"
                        ></path>
                    </defs>

                    <text
                        font-family="almarena"
                        font-size="18"
                        font-weight="medium"
                        letter-spacing="12px"
                        fill="#dad6ca"
                    >
                        <textPath xlink:href="#textPath">Open to new challanges</textPath>
                        <animateTransform
                            attributeName="transform"
                            type="rotate"
                            from="0 80 80"
                            to="360 80 80"
                            dur="13s"
                            repeatCount="indefinite"
                        ></animateTransform>
                    </text>
                </svg>
            </div>
        </div>
    }
}

