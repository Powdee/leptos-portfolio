use leptos::*;

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in w-full">
            <div class="lg:col-span-3">
                <div class="text-sm lg:text-md leading-about text-ek-white uppercase">
                    <span class="uppercase">Experience</span>
                </div>

            </div>
            <div class="lg:col-span-4 min-w-full text-xl lg:text-3xl leading-largep text-ek-white font-[400]">
                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span
                            data-text="Fullstack engineer/lead/architect"
                            class="justify-between flex-row w-full"
                        >
                            Freelancer
                            (c)
                            <small class="text-md text-ek-orange font-[400]">
                                "2019 – present"
                            </small>
                        </span>
                    </span>
                </div>

                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span
                            data-text="Senior Software Engineer"
                            class="justify-between flex-row w-full"
                        >
                            Splash Sports Inc. (c)
                            <small class="text-md text-ek-orange font-[400]">
                                "2023 – present"
                            </small>
                        </span>
                    </span>
                </div>

                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span
                            data-text="Lead Software Engineer"
                            class="justify-between flex-row w-full"
                        >
                            Alteryx (c)
                            <small class="text-md text-ek-orange font-[400]">"2019 – 2023"</small>
                        </span>
                    </span>
                </div>

                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span data-text="Software Engineer" class="justify-between flex-row w-full">
                            Generali
                            <small class="text-md text-ek-orange font-[400]">"2018 – 2019"</small>
                        </span>
                    </span>
                </div>

                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span data-text="Software Engineer" class="justify-between flex-row w-full">
                            Betsys
                            <small class="text-md text-ek-orange font-[400]">"2017 – 2018"</small>
                        </span>
                    </span>
                </div>

                <div class="experience experience-cta">
                    <span class="experience-cta-border"></span>
                    <span class="experience-cta-ripple">
                        <span></span>
                    </span>
                    <span class="experience-cta-title">
                        <span
                            data-text="Junior Software Developer"
                            class="justify-between flex-row w-full"
                        >
                            Tipsport
                            "a.s."
                            <small class="text-md text-ek-orange font-[400]">"2016 – 2017"</small>
                        </span>
                    </span>
                </div>

            </div>
        </div>
    }
}

