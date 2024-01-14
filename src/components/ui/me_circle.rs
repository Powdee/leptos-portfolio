use leptos::*;

#[component]
pub fn MeCircle() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 340 340"
            width="340"
            height="340"
            class="overflow-visible hidden lg:block"
        >
            <defs>
                <path
                    id="textPath"
                    d="M 170, 170
                    m -170, 0
                    a 170, 170 0 1,1 340,0
                    a 170,170 0 1,1 -340, 0"
                ></path>
            </defs>

            <text
                font-family="Jokker"
                font-size="24"
                font-weight="medium"
                letter-spacing="34px"
                fill="#6C757D"
            >
                <textPath xlink:href="#textPath">Open to new challenges</textPath>
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="0 170 170"
                    to="360 170 170"
                    dur="13s"
                    repeatCount="indefinite"
                ></animateTransform>
            </text>

        // <text
        // font-family="Jokker"
        // font-size="30"
        // font-weight="light"
        // letter-spacing="6px"
        // fill="#212529"
        // >
        // <textPath xlink:href="#textPath">Open to new challenges</textPath>
        // <animateTransform
        // attributeName="transform"
        // type="rotate"
        // from="150 150 150"
        // to="552 150 150"
        // dur="13s"
        // repeatCount="indefinite"
        // ></animateTransform>
        // </text>

        // <circle
        // stroke-width="2px"
        // cx="170"
        // cy="170"
        // r="150"
        // fill="none"
        // stroke="#212529"
        // ></circle>
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
                font-family="Jokker"
                font-size="21"
                font-weight="light"
                letter-spacing="6px"
                fill="#ADB5BD"
            >
                <textPath xlink:href="#textPathTablet">Open to new challenges</textPath>
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="0 120 120"
                    to="360 120 120"
                    dur="13s"
                    repeatCount="indefinite"
                ></animateTransform>
            </text>

            <text
                font-family="Jokker"
                font-size="21"
                font-weight="light"
                letter-spacing="6px"
                fill="#ADB5BD"
            >
                <textPath xlink:href="#textPathTablet">Open to new challenges</textPath>
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="192 120 120"
                    to="552 120 120"
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
                stroke="#ADB5BD"
            ></circle>
        </svg>
    }
}

