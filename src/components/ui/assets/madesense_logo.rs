use leptos::*;

#[component]
pub fn MadesenseLogo() -> impl IntoView {
    view! {
        <svg
            class="lg:w-[108px] w-[60px]"
            width="108"
            height="96"
            viewBox="0 0 108 96"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <g clip-path="url(#clip0_642_206)">
                <path
                    d="M97.5015 12.3193L54 37.48L10.4984 12.3193H97.5015Z"
                    stroke="#F8F9FA"
                    stroke-width="3"
                ></path>
                <path d="M4.90906 10.8193L54 95.9999" stroke="#F8F9FA" stroke-width="3"></path>
                <path d="M54 39.2129V96" stroke="#F8F9FA" stroke-width="3"></path>
                <path d="M103.091 10.8193L54 95.9999" stroke="white" stroke-width="3"></path>
            </g>
            <defs>
                <clipPath id="clip0_642_206">
                    <rect width="108" height="96" fill="white"></rect>
                </clipPath>
            </defs>
        </svg>
    }
}

