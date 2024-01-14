use leptos::*;

#[component]
pub fn Button(label: String, class_name: String) -> impl IntoView {
    let after_label = label.clone();

    view! {
        <button class=format!("button button-cta {}", class_name) role="button">
            <span class="button-cta-border"></span>
            <span class="button-cta-ripple">
                <span></span>
            </span>
            <span class="button-cta-title">
                <span data-text=label>{after_label}</span>
            </span>
        </button>
    }
}

