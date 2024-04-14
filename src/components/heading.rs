use leptos::*;

#[component]
pub fn Heading(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] title: String,
) -> impl IntoView {
    let classes = format!("{class} max-w-[50rem] mx-auto mb-12 lg:mb-20 md:text-center");
    view! {
        <div class=classes>
            <h2 class="h2">{title}</h2>
        </div>
    }
}
