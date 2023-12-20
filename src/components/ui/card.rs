use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn Card(
    name: &'static str,
    #[prop(default = "")] class_name: &'static str,
    #[prop(default = "")] style: &'static str,
    #[prop(default = false)] has_illustration: bool,
    #[prop(default = true)] is_link: bool,
) -> impl IntoView {
    let (is_hovered, set_hovered) = create_signal(false);
    let spacer = " ";

    view! {
        <Show when=move || { !is_link }>
            <div
                style=style
                class=class_name.to_owned() + spacer
                    + "cursor-pointer overflow-hidden relative bg-gray-2 rounded-[57px] duration-500 transition-shadow hover:shadow-md"
            >
                <div class="bg-gray-1 w-auto absolute left-5 bottom-5 rounded-full flex flex-row justify-center items-center"></div>
            </div>
        </Show>
        <Show when=move || { is_link }>
            <a
                href="projects/".to_owned() + name
                style=style
                on:mouseenter=move |_e: MouseEvent| set_hovered(true)
                on:mouseleave=move |_e: MouseEvent| set_hovered(false)
                id=name
                class=class_name.to_owned() + spacer
                    + "cursor-pointer overflow-hidden relative bg-gray-2 rounded-[57px] duration-500 transition-shadow hover:shadow-md"
            >
                <Show when=move || { has_illustration }>
                    <div
                        class="w-full h-full bg-cover bg-no-repeat bg-center rounded-[57px]"
                        style=format!("background-image: url(assets/{}.svg)", name)
                    ></div>
                </Show>
                <div class="bg-gray-1 w-auto absolute left-5 bottom-5 rounded-full flex flex-row justify-center items-center">
                    <div class="flex gap-4 flex-row items-center py-3 px-3">
                        <svg
                            width="48"
                            height="48"
                            viewBox="0 0 58 58"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <g clip-path="url(#clip0_359_60)">
                                <path
                                    d="M40.9647 17.0386L17.0385 40.9648"
                                    stroke="#212529"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                ></path>
                                <path
                                    d="M24.1677 16.9177L40.9647 17.0353L41.0854 33.8323"
                                    stroke="#212529"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                ></path>
                            </g>
                            <defs>
                                <clipPath id="clip0_359_60">
                                    <rect width="48" height="48" fill="#F8F9FA"></rect>
                                </clipPath>
                            </defs>
                        </svg>
                        <Show when=move || { is_hovered() }>
                            <span class="text-gray-9 pr-3 font-medium text-lg md:text-2xl">
                                {name}
                            </span>
                        </Show>
                    </div>
                </div>
            </a>
        </Show>
    }
}

