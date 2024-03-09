use crate::components::ui::layout::Layout;

use gloo_timers::callback::Interval;
use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    let (skill, write_skill) = create_signal("TypeScript");

    create_effect(move |_| {
        let items = vec![
            "TypeScript",
            "Rust",
            "Docker",
            "Jest",
            "NodeJS",
            "AWS",
            "goJS",
            "NextJS",
            "JavaScript",
        ];
        let clone_items = items.clone();
        let timer = Interval::new(2000, move || {
            let upcoming_skill = clone_items
                .iter()
                .cycle()
                .skip_while(|&s| s != &skill.get_untracked())
                .skip(1)
                .next()
                .unwrap_or(&"TypeScript");

            write_skill(upcoming_skill);
        });

        move || timer.forget()
    });

    view! {
        <div class="absolute backdrop-blur z-0 top-0 h-[50svh] lg:h-screen w-full bg-ek-dark bg-dot-ek-orange/[0.7] flex items-center justify-center">
            <div class="absolute pointer-events-none inset-0 flex items-center justify-center bg-ek-dark [mask-image:radial-gradient(ellipse_at_center,transparent_20%,#0e0306)]"></div>
        </div>
        <Layout id="home".to_string() aria_label="Hero" class_name="".to_string()>
            <h1 class="text-center w-full">
                <div class="animated-title">
                    <span class="text-6xl sm:text-8xl lg:text-9xl xl:text-10xl text-ek-white animated-title-element font-bold uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading  sm:tracking-heading">
                        "Leading"
                    </span>
                </div>
                <br/>
                <div class="animated-title text-center">
                    <span class="text-5xl sm:text-8xl lg:text-9xl xl:text-10xl text-ek-orange animated-title-element font-bold break-all uppercase leading-smallheading sm:leading-mediumheading  xl:leading-heading tracking-smallheading sm:tracking-heading">
                        "Engineering"
                    </span>
                    {move || {
                        view! {
                            <span class="sm:block hidden animated-flip-up absolute text-ek-orange top-[-30px] z-100 right-0 text-sm md:text-md lg:text-xl">
                                {format!("({})", skill.get())}
                            </span>
                        }
                    }}

                </div>
                <br/>

                <div class="animated-title">
                    <span class="text-6xl sm:text-8xl lg:text-9xl xl:text-10xl text-ek-white animated-title-element font-bold uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading relative">
                        "Innovation"
                    </span>
                </div>
                <br/>

                <div class="animated-title">
                    <div class="animated-title-element">
                        <span class="text-3xl sm:text-6xl lg:text-7xl text-ek-orange font-[400] uppercase leading-p sm:leading-mediump lg:leading-heading xl:leading-heading tracking-p relative mr-4 xs:mr-12">
                            "with"
                        </span>
                        <br class="block md:hidden"/>
                        <span class="text-6xl sm:text-8xl lg:text-9xl xl:text-10xl text-ek-white font-bold uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading ">
                            "Vision"
                        </span>
                    </div>
                </div>
            </h1>
            <div class="absolute hidden md:block left-5 -bottom-20">
                <span class="scroll-icon hero">
                    <span class="scroll-icon__dot"></span>
                </span>
            </div>
        </Layout>
    }
}

