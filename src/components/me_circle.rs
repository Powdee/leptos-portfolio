use leptos::*;

#[component]
pub fn MeCircle() -> impl IntoView {
    view! {
        <svg
            viewBox="0 0 384 384"
            width="384"
            height="384"
            class="overflow-visible hidden lg:block"
        >
            <defs>
                <path
                    id="textPath"
                    d="M 192, 192
                    m -192, 0
                    a 192, 192 0 1,1 384,0
                    a 192,192 0 1,1 -384, 0"
                ></path>
            </defs>

            <text font-family="Jokker" font-size="30" fill="#000">
                <textPath xlink:href="#textPath">Open to new challenges</textPath>
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="0 192 192"
                    to="360 192 192"
                    dur="10s"
                    repeatCount="indefinite"
                ></animateTransform>
            </text>

            <text font-family="Jokker" font-size="30" fill="#000">
                <textPath xlink:href="#textPath">Open to new challenges</textPath>
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="192 192 192"
                    to="552 192 192"
                    dur="10s"
                    repeatCount="indefinite"
                ></animateTransform>
            </text>

            <circle stroke-width="2px" cx="192" cy="192" r="180" fill="none" stroke="#000"></circle>
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

            <text font-family="Jokker" font-size="21" fill="#000">
                <textPath xlink:href="#textPathTablet">Open to new challenges</textPath>
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="0 120 120"
                    to="360 120 120"
                    dur="10s"
                    repeatCount="indefinite"
                ></animateTransform>
            </text>

            <text font-family="Jokker" font-size="21" fill="#000">
                <textPath xlink:href="#textPathTablet">Open to new challenges</textPath>
                <animateTransform
                    attributeName="transform"
                    type="rotate"
                    from="192 120 120"
                    to="552 120 120"
                    dur="10s"
                    repeatCount="indefinite"
                ></animateTransform>
            </text>

            <circle stroke-width="1px" cx="120" cy="120" r="110" fill="none" stroke="#000"></circle>
        </svg>
    }
}

