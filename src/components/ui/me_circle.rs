use leptos::*;

#[component]
pub fn MeCircle() -> impl IntoView {
    view! {
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
    }
}

#[component]
pub fn MeCircleTablet() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 240 240"
            width="240"
            height="240"
            class="overflow-visible block lg:hidden"
        >
            <defs>
                <path
                    id="textPathTablet"
                    d="M 120, 120
                    m -120, 0
                    a 120, 120 0 1,1 240,0
                    a 120,120 0 1,1 -240, 0"
                ></path>
            </defs>

            <text
                font-family="almarena"
                font-size="21"
                font-weight="light"
                letter-spacing="20px"
                fill="#dad6ca"
            >
                <textPath xlink:href="#textPathTablet">Open to new challanges</textPath>
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="0 120 120"
                    to="360 120 120"
                    dur="13s"
                    repeatCount="indefinite"
                ></animateTransform>
            </text>

            <circle
                stroke-width="1px"
                cx="120"
                cy="120"
                r="110"
                fill="none"
                stroke="#dad6ca"
            ></circle>
        </svg>
    }
}

