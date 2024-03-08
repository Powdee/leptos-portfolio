use leptos::*;

#[component]
pub fn Card(
    children: Children,
    name: String,
    #[prop(into)] class_name: String,
    #[prop(into)] style: String,
) -> impl IntoView {
    let spacer = " ";

    view! {
        <a
            href=format!("projects/{}", name)
            style=style
            id=name
            class=class_name.to_owned() + spacer
                + "cursor-pointer overflow-hidden relative bg-ek-white rounded-[16px] md:rounded-[16px] duration-500 transition-shadow hover:shadow-md min-h-[200px]"
        >
            {children()}
        </a>
    }
}

