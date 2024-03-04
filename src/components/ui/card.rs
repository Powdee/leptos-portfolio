use leptos::{html::A, *};
use leptos_use::use_element_hover;

#[component]
pub fn Card(
    children: Children,
    name: String,
    #[prop(into)] class_name: String,
    #[prop(into)] style: String,
) -> impl IntoView {
    let el = create_node_ref::<A>();
    let is_hovered = use_element_hover(el);

    let spacer = " ";
    let name_cloned = name.clone();

    view! {
        <a
            href=format!("projects/{}", name)
            style=style
            id=name
            node_ref=el
            class=class_name.to_owned() + spacer
                + "cursor-pointer overflow-hidden relative bg-ek-white rounded-[47px] md:rounded-[57px] duration-500 transition-shadow hover:shadow-md min-h-[200px]"
        >

            {children()}
            <div class=
            {
                let base_class = "bg-ek-dark w-auto absolute left-5 bottom-5 rounded-full flex flex-row justify-center items-center ease-out duration-700 hidden lg:block";
                move || {
                    if is_hovered.get() {
                        format!("{} {}", base_class, "shadow-explore")
                    } else {
                        format!("{} {}", base_class, "shadow-none")
                    }
                }
            }>

                <div class="flex flex-row items-center py-3 px-3">
                    <div class={
                        let base_class = "transition-all duration-700 linear overflow-hidden";
                        move || {
                            if is_hovered.get() {
                                format!(
                                    "{} {}",
                                    base_class,
                                    "max-w-[200px] pl-3 pr-3 translate-x-0",
                                )
                            } else {
                                format!("{} {}", base_class, "max-w-0 -translate-x-4")
                            }
                        }
                    }>
                        <span class={
                            let base_class = "text-ek-white font-[400] text-lg md:text-2xl ease-out duration-500 transition-opacity";
                            move || {
                                if is_hovered.get() {
                                    format!("{} {}", base_class, "opacity-100 delay-300")
                                } else {
                                    format!("{} {}", base_class, "opacity-0")
                                }
                            }
                        }>{name_cloned}</span>
                    </div>
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
                                stroke="#0e0306"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></path>
                            <path
                                d="M24.1677 16.9177L40.9647 17.0353L41.0854 33.8323"
                                stroke="#0e0306"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></path>
                        </g>
                        <defs>
                            <clipPath id="clip0_359_60">
                                <rect width="48" height="48" fill="#0e0306"></rect>
                            </clipPath>
                        </defs>
                    </svg>

                </div>
            </div>
        </a>
    }
}

