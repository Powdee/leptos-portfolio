use leptos::svg::Svg;
use leptos::*;
use leptos_meta::*;
use leptos_use::use_element_hover;

use crate::components::ui::button::Button;
use crate::components::ui::close::Close;
use crate::components::ui::layout::Layout;

#[component]
pub fn DownloadCVPage() -> impl IntoView {
    let closeIcon = create_node_ref::<Svg>();
    let is_hovered = use_element_hover(closeIcon);

    view! {
        <Title text="Erik Kurjak - Résumé"/>
        <Close el=closeIcon/>
        <main class={
            let base_class = "grid gap-20 md:gap-28 lg:gap-64 mt-10 xl:mt-28 delay-75 duration-1000 ease-out";
            move || {
                if is_hovered.get() {
                    format!("{} {}", base_class, "usecase-in")
                } else {
                    format!("{} {}", base_class, "usecase-out")
                }
            }
        }>
            <Layout
                id="resume".to_string()
                aria_label="resume"
                class_name="flex-col mb-10 xl:mb-28".to_string()
            >
                <div class="flex flex-col lg:flex-row gap-16 md:gap-28">
                    <div class="relative order-2 lg:order-1 basis-[60%] fade-y-trans">
                        <div class="absolute hidden md:block left-1/2 top-4 transform -translate-x-1/2">
                            <section class="example example--2">
                                <span class="scroll-icon">
                                    <span class="scroll-icon__dot"></span>
                                </span>
                            </section>
                        </div>
                        <iframe
                            class="iframe"
                            frameborder="0"
                            allowfullscreen
                            src="https://resume.erikkurjak.com/embed-resume.html"
                        ></iframe>
                    </div>

                    <div class="basis-[40%] order-1 lg:order-2">
                        <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl text-ek-white leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading">
                            <div class="animated-title">
                                <em class="animated-title-element text-ek-white font-bold uppercase">
                                    My
                                </em>
                            </div>
                            <br/>
                            <div class="animated-title">
                                <span class="animated-title-element text-ek-white font-bold uppercase">
                                    résumé
                                </span>
                            </div>
                        </h1>
                        <br/>
                        <p class="text-xl md:text-2xl lg:text-3xl leading-p lg:leading-largep text-ek-white fade-y-trans">
                            Résumé is coded in a LaTex and generated from Makefile using a Github actions and hosted on
                            <i>fly.io</i> .
                            You can find the source code
                            <a
                                href="https://github.com/Powdee/leptos-cv-resume"
                                target="_blank"
                                class="font-bold"
                            >
                                here
                            </a> .
                        </p>
                        <br/>
                        <br/>
                        <Button
                            href="https://resume.erikkurjak.com/resume.pdf".to_string()
                            class_name="".to_string()
                            label="Download PDF".to_string()
                        />
                    </div>
                </div>
            </Layout>
        </main>
    }
}

