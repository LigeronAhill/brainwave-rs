use leptos::*;

use crate::components::tagline::TagLine;

#[component]
pub fn Heading(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] tag: Option<String>,
) -> impl IntoView {
    let classes = format!("{class} max-w-[50rem] mx-auto mb-12 lg:mb-20 md:text-center");
    view! {
        <div class=classes>
            {tag
                .map(|t| {
                    view! { <TagLine class="mb-4 md:justify-center">{t}</TagLine> }
                })}
            {title
                .map(|t| {
                    view! { <h2 class="h2">{t}</h2> }
                })}
            {text
                .map(|t| {
                    view! { <p class="body2 mt-4 text-n-4">{t}</p> }
                })}

        </div>
    }
}
