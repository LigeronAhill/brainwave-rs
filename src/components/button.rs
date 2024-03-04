use leptos::*;
use web_sys::MouseEvent;

use crate::components::svg::ButtonSvg;

#[allow(non_snake_case)]
#[component]
pub fn Button(
    #[prop(default = "".to_string())] class: String,
    #[prop(optional)] href: Option<String>,
    // #[prop(into)] on_click: Callback<MouseEvent>,
    children: Children,
    #[prop(default = "px-7".to_string())] px: String,
    #[prop(default = false)] white: bool,
) -> impl IntoView {
    let text = if white { "text-n-8" } else { "text-n-1" };
    let classes = format!("button relative inline-flex items-center justify-center h-11 transition-colors hover:text-color-1 {px} {text} {class}");
    let span_classes = "relative z-10";
    match href {
        Some(href) => view! {
            <a href=href class=classes>
                <span class=span_classes>{children()}</span>
                <ButtonSvg white=white/>
            </a>
        }
        .into_view(),
        None => view! {
            <button class=classes>
                // <button class=classes on:click=on_click>
                <span class=span_classes>{children()}</span>
                <ButtonSvg white=white/>
            </button>
        }
        .into_view(),
    }
}
