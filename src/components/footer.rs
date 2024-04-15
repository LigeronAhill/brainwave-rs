use chrono::Datelike;
use leptos::*;

use crate::{components::section::Section, constants::SOCIALS};

#[component]
pub fn Footer() -> impl IntoView {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    let text = format!("© {year}. All rights reserved.");
    view! {
        <Section crosses=true class_name="!px-0 !py-10">
            <div class="container flex sm:justify-between justify-center items-center gap-10 max-sm:flex-col">
                <p class="caption text-n-4 lg:block">{text}</p>

                <ul class="flex gap-5 flex-wrap">
                    {SOCIALS
                        .into_iter()
                        .map(|item| {
                            view! {
                                <a
                                    key=item.id
                                    href=item.url
                                    target="_blank"
                                    class="flex items-center justify-center w-10 h-10 bg-n-7 rounded-full transition-colors hover:bg-n-6"
                                >
                                    <img src=item.icon width=16 height=16 alt=item.title/>

                                </a>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
        </Section>
    }
}
