use leptos::*;

#[component]
pub fn MeCircle() -> impl IntoView {
    view! {
        <svg viewBox="0 0 384 384" width="384" height="384" class="overflow-visible">
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

            <circle stroke-width="3px" cx="192" cy="192" r="180" fill="none" stroke="#000"></circle>
            <circle stroke-width="1px" cx="192" cy="192" r="180" fill="#fff"></circle>
        </svg>
    }
}

