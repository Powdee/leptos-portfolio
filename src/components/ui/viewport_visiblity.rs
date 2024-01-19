use leptos::{html::Div, *};
use leptos_use::use_element_visibility;

// This have to a component and not an island
// because of possibility of rendering multiples children
// I needed to use ChildrenFn instead of Children
#[component]
pub fn ViewportVisibility(
    children: ChildrenFn,
    #[prop(optional, into)] fallback: Option<ViewFn>,
) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let is_element_visible = use_element_visibility(el);
    let (is_user_in_viewport, set_visibility) = create_signal(false);

    let is_users_first_time = create_memo(move |_| {
        is_element_visible.get() && !is_user_in_viewport.get()
    });

    create_effect(move |_| {
        if is_users_first_time.get() {
            set_visibility(true);
        }
    });

    view! {
        <div node_ref=el>
            <Show
                when=move || { is_users_first_time.get() || is_user_in_viewport.get() }
                fallback=fallback.unwrap_or_default()
            >
                {children()}
            </Show>
        </div>
    }
}

