use gloo_timers::callback::Interval;
use leptos::*;
use rand::random;

#[component]
pub fn Skills() -> impl IntoView {
    let technologies = vec![
        "TypeScript",
        "JavaScript",
        "React",
        "NextJS",
        "NodeJS",
        "Express",
        "Rest API",
        "Svelte",
        "Angular",
        "PostgresSQL",
        "Rust",
        "GraphQL",
        "Axum",
        "Jest",
        "Cypress",
        "AWS",
        "CI/DI",
        "Docker",
        "Management",
        "Leadership",
        "Micro-Frontends",
        "Micro-Services",
        "SOLID",
        "TDD",
        "Architecture",
    ];

    let (highlighted, write_highlighted) =
        create_signal(vec![false; technologies.len()]);

    create_effect(move |_| {
        let timer = Interval::new(1250, move || {
            let new_highlighted = (0..highlighted.get_untracked().len())
                .map(|index| {
                    if index % 3 == 0 {
                        random::<bool>()
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>();

            write_highlighted(new_highlighted);
        });

        move || timer.forget()
    });

    view! {
        <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in w-full">
            <div class="lg:col-span-2">
                <div class="text-sm lg:text-md leading-about text-ek-white uppercase">
                    <span class="uppercase">Skills</span>
                </div>

            </div>
            <div class="lg:col-span-5 py-5 lg:py-10 border-solid border-t border-b border-ek-white flex flex-wrap gap-x-4 gap-y-2 lg:gap-x-10 lg:gap-y-4 min-w-full text-xl lg:text-5xl leading-p lg:leading-largep font-[400]">
                {move || {
                    technologies
                        .iter()
                        .enumerate()
                        .map(|(index, tech)| {
                            let is_highlighted = highlighted.get()[index];
                            let class = if is_highlighted {
                                "text-ek-orange"
                            } else {
                                "text-ek-white"
                            };
                            view! { <span class=move || class>{tech.to_string()}</span> }
                        })
                        .collect::<Vec<_>>()
                }}

            </div>
        </div>
    }
}

