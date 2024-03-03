use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <Layout id="experience".to_string() aria_label="Experience" class_name="".to_string()>
            <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 gap-y-12 fade-in w-full">
                <div class="col-span-3">
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
                                data-text="Senior Software Engineer"
                                class="justify-between flex-row w-full"
                            >
                                Splash Sports Inc.
                                <small class="text-md md:text-lg text-ek-orange">
                                    2023-current
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
                                Alteryx
                                <small class="text-md md:text-lg text-ek-orange">
                                    2023-current
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
                                data-text="Software Engineer"
                                class="justify-between flex-row w-full"
                            >
                                Generali
                                <small class="text-md md:text-lg text-ek-orange">
                                    2023-current
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
                                data-text="Software Engineer"
                                class="justify-between flex-row w-full"
                            >
                                Betsys
                                <small class="text-md md:text-lg text-ek-orange">
                                    2023-current
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
                                data-text="Junior Software Developer"
                                class="justify-between flex-row w-full"
                            >
                                Tipsport
                                "as"
                                <small class="text-md md:text-lg text-ek-orange">
                                    2023-current
                                </small>
                            </span>
                        </span>
                    </div>

                </div>
            </div>
        </Layout>
    }
}

