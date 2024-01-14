use leptos::*;

#[component]
pub fn Button(label: String) -> impl IntoView {
    let after_label = label.clone();

    view! {
        <button class="button button-cta" role="button">
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

