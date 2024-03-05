use leptos::*;

use crate::components::svg::SectionSvg;
#[component]
pub fn Section(
    #[prop(optional, into)] class_name: String,
    #[prop(optional, into)] id: String,
    #[prop(optional)] crosses: bool,
    #[prop(optional, into)] crosses_offset: String,
    #[prop(optional)] custom_paddings: bool,
    children: Children,
) -> impl IntoView {
    let pad = if custom_paddings {
        ""
    } else {
        if crosses {
            "py-10 lg:py-16 xl:py-20 lg:py-32 xl:py-40"
        } else {
            "py-10 lg:py-16 xl:py-20"
        }
    };
    view! {
        <div id=id class=format!("relative {pad} {class_name}")>
            {children()}
            <div class="hidden absolute top-0 left-5 w-0.25 h-full bg-stroke-1 pointer-events-none md:block lg:left-7.5 xl:left-10"></div>
            <div class="hidden absolute top-0 right-5 w-0.25 h-full bg-stroke-1 pointer-events-none md:block lg:right-7.5 xl:right-10"></div>

            if crosses

            {
                view! {
                    <div class=format!(
                        "hidden absolute top-0 left-7.5 right-7.5 h-0.25 bg-stroke-1 pointer-events-none lg:block xl:left-10 right-10 {crosses_offset}",
                    )></div>
                    <SectionSvg crosses_offset=crosses_offset/>
                }
            }

        </div>
    }
}
