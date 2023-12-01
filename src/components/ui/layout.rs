use leptos::*;

#[component]
pub fn Layout(
    children: Children,
    aria_label: &'static str,
    #[prop(default = "")] class_name: &'static str,
) -> impl IntoView {
    let spacer = " ";

    view! {
        <section
            aria-label=aria_label
            class=class_name.to_owned() + spacer
                + "relative w-full isolate lg:mx-auto max-w-full lg:mx-0 lg:flex lg:max-w-none"
        >

            {children()}
        </section>
    }
}

