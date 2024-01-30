use leptos::*;

#[component]
pub fn CardLink(
    children: Children,
    #[prop(default = "")] class_name: &'static str,
    #[prop(default = "")] style: &'static str,
) -> impl IntoView {
    let spacer = " ";

    view! {
        <div
            style=style
            class=class_name.to_owned() + spacer
                + "overflow-hidden relative bg-gray-9 rounded-[57px] duration-500 transition-shadow hover:shadow-md"
        >
            <div class="w-full h-full rounded-full flex flex-row justify-center items-center">
                {children()}
            </div>
        </div>
    }
}

