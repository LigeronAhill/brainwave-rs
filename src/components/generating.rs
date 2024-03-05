use leptos::*;

#[component]
pub fn Generating(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class=format!(
            "flex items-center h-[3.5rem] px-6 bg-n-8/80 rounded-[1.7rem] {class} text-base",
        )>
            <img src="/public/loading.png" alt="loading" class="2-5 h-5 mr-4"/>
            "AI is generating"
        </div>
    }
}
