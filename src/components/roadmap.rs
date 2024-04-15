use leptos::*;

use crate::{
    components::{section::Section, tagline::TagLine},
    constants::{Roadmap, ROADMAP},
};

use super::{button::Button, heading::Heading, services::Gradient};

#[component]
pub fn Roadmap() -> impl IntoView {
    view! {
        <Section class_name="overflow-hidden" id="roadmap">
            <div class="container md:pb-10">
                <Heading tag="Ready to get started" title="What we’re working on"/>

                <div class="relative grid gap-6 md:grid-cols-2 md:gap-4 md:pb-[7rem]">
                    {ROADMAP
                        .into_iter()
                        .map(|item| {
                            view! { <RoadmapItem item=item/> }
                        })
                        .collect_view()} <Gradient/>
                </div>

                <div class="flex justify-center mt-12 md:mt-15 xl:mt-20">
                    <Button href="/roadmap".to_string()>"Our roadmap"</Button>
                </div>
            </div>
        </Section>
    }
}
#[component]
fn RoadmapItem(item: Roadmap<'static>) -> impl IntoView {
    let (status, img) = if item.status == "done" {
        ("Done", "/public/check-02.svg")
    } else {
        ("In progress", "public/loading-01.svg")
    };
    let colorful = if item.colorful {
        "bg-conic-gradient"
    } else {
        "bg-n-6"
    };
    view! {
        <div
            class=format!("md:flex even:md:translate-y-[7rem] p-0.25 rounded-[2.5rem] {colorful}")
            key=item.id
        >
            <div class="relative p-8 bg-n-8 rounded-[2.4375rem] overflow-hidden xl:p-15">
                <div class="absolute top-0 left-0 max-w-full">
                    <img class="w-full" src="/public/grid.png" width=550 height=550 alt="Grid"/>
                </div>
                <div class="relative z-1">
                    <div class="flex items-center justify-between max-w-[27rem] mb-8 md:mb-20">
                        <TagLine>{item.date}</TagLine>

                        <div class="flex items-center px-4 py-1 bg-n-1 rounded text-n-8">
                            <img class="mr-2.5" src=img width=16 height=16 alt=status/>
                            <div class="tagline">{status}</div>
                        </div>
                    </div>

                    <div class="mb-10 -my-10 -mx-15">
                        <img class="w-full" src=item.image_url width=628 height=426 alt=item.title/>
                    </div>
                    <h4 class="h4 mb-4">{item.title}</h4>
                    <p class="body-2 text-n-4">{item.text}</p>
                </div>
            </div>
        </div>
    }
}
