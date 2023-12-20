use crate::components::ui::card::Card;
use crate::components::ui::layout::Layout;
use leptos::*;
use leptos_router::use_params_map;
use web_sys::MouseEvent;

#[component]
pub fn UseCasePage() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params.with(|params| params.get("id").cloned().unwrap_or_default())
    };

    let (is_hovered, set_hovered) = create_signal(false);

    view! {
        <header class="mx-auto max-w-full py-6 px-10 md:py-12 md:px-16">
            <nav class="gap-2 md:flex-row flex-col flex items-center justify-center" aria-label="X">
                <a href="/">
                    <svg
                        on:mouseenter=move |_e: MouseEvent| set_hovered(true)
                        on:mouseleave=move |_e: MouseEvent| set_hovered(false)
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
        <main class={
            let base_class = "flex delay-75 duration-1000 mb-16 ease-out";
            move || {
                if is_hovered.get() {
                    format!("{} {}", base_class, "usecase-in")
                } else {
                    format!("{} {}", base_class, "usecase-out")
                }
            }
        }>
            <Layout aria_label="Usecase" class_name="flex-col">
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-normal text-gray-9 leading-tighter font-regular mb-4 mt-8 md:mb-10 md:mt-20">
                    {id}
                </h1>
                <div class="flex flex-col md:flex-row gap-8 md:gap-10 lg:gap-20">
                    <div class="flex flex-col gap-8">
                        <p class="text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9">
                            A team player with a passion for building modern digital solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                        </p>

                        <div class="flex flex-row gap-2 md:gap-4 overflow-x-scroll md:overflow-x-hidden">
                            <div class="bg-gray-9 rounded-full px-6 py-2 cursor-pointer">
                                <span class="font-medium text-md text-gray-1">visit</span>
                            </div>
                            <div class="bg-gray-3 rounded-full px-6 py-2">
                                <span class="font-medium text-md">iOS</span>
                            </div>
                            <div class="bg-gray-3 rounded-full px-6 py-2">
                                <span class="font-medium text-md">android</span>
                            </div>
                            <div class="bg-gray-3 rounded-full px-6 py-2">
                                <span class="font-medium text-md">flutter</span>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-col gap-2 md:gap-8">
                        <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9">
                            A team player with a <b>passion for building</b>
                            modern digital solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                        </p>
                        <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9">
                            A team player with a passion for building <b>modern digital</b>
                            solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                        </p>
                        <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9">
                            A team player with a passion for building <b>modern digital</b>
                            solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                        </p>
                    </div>
                </div>

                <div class="grid gap-4 md:grid-cols-5 md:grid-rows-7 mt-10 md:mt-20">
                    <Card
                        is_link=false
                        name="alteryx"
                        class_name="md:col-span-3 md:row-span-3 min-h-card_1_row_mobile md:min-h-card_1_row"
                    />
                    <Card
                        is_link=false
                        name="splashsports"
                        class_name="md:col-span-2 md:row-span-3 min-h-card_1_row_mobile md:min-h-card_1_row"
                    />
                    <Card
                        is_link=false
                        name="oms"
                        class_name="md:col-span-2 md:row-span-4 min-h-card_2_row_mobile md:min-h-card_2_row"
                    />
                    <Card
                        is_link=false
                        name="helpie"
                        class_name="md:col-span-3 md:row-span-2 min-h-card_2_row_mobile"
                    />
                    <Card
                        is_link=false
                        name="invaders"
                        class_name="md:col-span-1 md:row-span-2 bg-gray-8 min-h-card_2_row_mobile"
                    />
                    <Card
                        is_link=false
                        name="madesense"
                        class_name="md:col-span-2 md:row-span-2 min-h-card_2_row_mobile"
                    />
                </div>
            </Layout>
        </main>
    }
}

