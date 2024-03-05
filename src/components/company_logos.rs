use leptos::*;

use crate::constants::COMPANY_LOGOS;

#[component]
pub fn CompanyLogos(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div class=class>
            <h5 class="tagline mb-6 text-center text-n-1/50">
                "Helping people create beautiful content at"
            </h5>
            <ul class="flex">
                {COMPANY_LOGOS
                    .into_iter()
                    .enumerate()
                    .map(|(index, logo)| {
                        view! {
                            <li
                                class="flex items-center justify-center flex-1 h-[8.5rem]"
                                key=index
                            >
                                <img src=logo width=134 height=28 alt=logo/>
                            </li>
                        }
                    })
                    .collect_view()}

            </ul>
        </div>
    }
}
