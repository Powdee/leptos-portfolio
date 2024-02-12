use leptos::svg::Svg;
use leptos::*;

#[component]
pub fn Close(el: NodeRef<Svg>) -> impl IntoView {
    view! {
        <header class="mx-auto max-w-full py-6 px-10 md:py-12 md:px-16">
            <nav class="gap-2 md:flex-row flex-col flex items-center justify-center" aria-label="X">
                <a href="/">
                    <svg
                        node_ref=el
                        width="61"
                        height="61"
                        class="cursor-pointer hover:scale-105 ease-out duration-300 close-x"
                        viewBox="0 0 61 61"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <rect
                            width="61"
                            height="61"
                            rx="30.5"
                            class="fill-gray-2 hover:fill-gray-3 ease-out duration-300"
                        ></rect>
                        <g clip-path="url(#clip0_615_114)">
                            <path
                                d="M38.7751 24.0174L36.9825 22.2248L29.8756 29.3317L22.7687 22.2248L20.9761 24.0174L28.083 31.1243L20.9761 38.2312L22.7687 40.0238L29.8756 32.9169L36.9825 40.0238L38.7751 38.2312L31.6682 31.1243L38.7751 24.0174Z"
                                fill="#212529"
                            ></path>
                        </g>
                        <defs>
                            <clipPath id="clip0_615_114">
                                <rect
                                    width="51.7791"
                                    height="51.7791"
                                    fill="white"
                                    transform="translate(4.25586 4.96509)"
                                ></rect>
                            </clipPath>
                        </defs>
                    </svg>
                </a>
            </nav>
        </header>
    }
}
