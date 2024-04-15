use leptos::*;

use crate::components::{heading::Heading, pricing_list::PricingList, section::Section};

#[component]
pub fn Pricing() -> impl IntoView {
    view! {
        <Section class_name="overflow-hidden" id="pricing">
            <div class="container relative z-2">
                <div class="hidden relative justify-center mb-[6.5rem] lg:flex">
                    <img
                        src="/public/4-small.png"
                        class="relative z-1"
                        width=255
                        height=255
                        alt="Sphere"
                    />
                    <div class="absolute top-1/2 left-1/2 w-[60rem] -translate-x-1/2 -translate-y-1/2 pointer-events-none">
                        <img
                            src="/public/pricing/stars.svg"
                            class="w-full"
                            width=950
                            height=400
                            alt="Stars"
                        />
                    </div>
                </div>

                <Heading tag="Get started with Brainwave" title="Pay once, use forever"/>

                <div class="relative">
                    <PricingList/>
                    <LeftLine/>
                    <RightLine/>
                </div>

                <div class="flex justify-center mt-10">
                    <a
                        class="text-xs font-code font-bold tracking-wider uppercase border-b"
                        href="/pricing"
                    >
                        "See the full details"
                    </a>
                </div>
            </div>
        </Section>
    }
}

#[component]
pub fn LeftLine() -> impl IntoView {
    view! {
        <div class="hidden lg:block absolute top-1/2 right-full w-[92.5rem] h-[11.0625rem] -translate-y-1/2 pointer-events-none">
            <img class="w-full" src="/public/pricing/lines.svg" width=1480 height=177 alt="Lines"/>
        </div>
    }
}
#[component]
pub fn RightLine() -> impl IntoView {
    view! {
        <div class="hidden lg:block absolute top-1/2 left-full w-[92.5rem] h-[11.0625rem] -translate-y-1/2 -scale-x-100 pointer-events-none">
            <img class="w-full" src="/public/pricing/lines.svg" width=1480 height=177 alt="Lines"/>
        </div>
    }
}
