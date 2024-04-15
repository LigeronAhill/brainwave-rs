use leptos::*;

use crate::{
    components::button::Button,
    constants::{Pricing, PRICING},
};

#[component]
pub fn PricingList() -> impl IntoView {
    view! {
        <div class="flex gap-[1rem] max-lg:flex-wrap">
            {PRICING
                .into_iter()
                .map(|item| {
                    view! { <PricingItem item=item/> }
                })
                .collect_view()}
        </div>
    }
}
#[component]
fn PricingItem(item: Pricing<'static>) -> impl IntoView {
    let white = item.price.is_some();
    let href = if item.price.is_some() {
        "/pricing".to_string()
    } else {
        "mailto:contact@jsmastery.pro".to_string()
    };
    let button_text = if item.price.is_some() {
        "Get started"
    } else {
        "Contact us"
    };
    view! {
        <div
            key=item.id.to_string()
            class="w-[19rem] max-lg:w-full h-full px-6 bg-n-8 border border-n-6 rounded-[2rem] lg:w-auto even:py-14 odd:py-8 odd:my-4 [&>h4]:first:text-color-2 [&>h4]:even:text-color-1 [&>h4]:last:text-color-3"
        >
            <h4 class="h4 mb-4">{item.title.to_string()}</h4>

            <p class="body-2 min-h-[4rem] mb-3 text-n-1/50">{item.description.to_string()}</p>

            <div class="flex items-center h-[5.5rem] mb-6">
                {item
                    .price
                    .map(|p| {
                        view! {
                            <div class="h3">"$"</div>
                            <div class="text-[5.5rem] leading-none font-bold">{p.to_string()}</div>
                        }
                    })}

            </div>

            <Button class="w-full mb-6".to_owned() href=href white=white>
                {button_text}
            </Button>

            <ul>
                {item
                    .features
                    .into_iter()
                    .enumerate()
                    .map(|(index, feature)| {
                        view! {
                            <li key=index class="flex items-start py-5 border-t border-n-6">
                                <img src="/public/check.svg" width=24 height=24 alt="Check"/>
                                <p class="body-2 ml-4">{feature.to_string()}</p>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
