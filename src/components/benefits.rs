use leptos::*;

use crate::{
    components::{
        design::{arrow::Arrow, clip_path::CustomClipPath, gradient_light::GradientLight},
        heading::Heading,
        section::Section,
    },
    constants::BENEFITS,
};

#[component]
pub fn Benefits() -> impl IntoView {
    view! {
        <Section id="features">
            <div class="container relative z-2">
                <Heading
                    class="md:max-w-md lg:max-w-2xl"
                    title="Chat Smarter, Not Harder with Brainwave"
                />
            </div>
            <div class="flex flex-wrap gap-10 mb-10">
                {BENEFITS
                    .into_iter()
                    .map(|b| {
                        let style = format!("background-image: url('{}');", b.background_url);
                        if b.light {
                            view! {
                                <div
                                    class="block relative p-0.5 bg-no-repeat bg-[length:100%_100%] md:max-w-[24rem]"
                                    key=b.id
                                    style=style
                                >
                                    <div class="relative z-2 flex flex-col min-h-[22rem] p-[2.4rem] pointer-events-none">
                                        <h5 class="h5 mb-5">{b.title}</h5>
                                        <p class="body-2 mb-6 text-n-3">{b.text}</p>
                                        <div class="flex items-center mt-auto">
                                            <img src=b.icon_url alt=b.title width=48 height=48/>
                                            <p class="ml-auto font-code text-xs font-bold text-n-1 uppercase tracking-wider">
                                                "Explore more"
                                            </p>
                                            <Arrow/>
                                        </div>
                                    </div>
                                    <GradientLight/>
                                    <div
                                        class="absolute inset-0 5 bg-n-8"
                                        style=r#"clip-path: url(#benefits)"#
                                    >
                                        <div class="absolute inset-0 opacity-0 transition-opacity hover:opacity-10">
                                            <img
                                                class="w-full h-full object-cover"
                                                src=b.image_url
                                                width=380
                                                height=362
                                                alt=b.title
                                            />
                                        </div>
                                    </div>
                                    <CustomClipPath/>
                                </div>
                            }
                        } else {
                            view! {
                                <div
                                    class="block relative p-0.5 bg-no-repeat bg-[length:100%_100%] md:max-w-[24rem]"
                                    key=b.id
                                    style=style
                                >
                                    <div class="relative z-2 flex flex-col min-h-[22rem] p-[2.4rem] pointer-events-none">
                                        <h5 class="h5 mb-5">{b.title}</h5>
                                        <p class="body-2 mb-6 text-n-3">{b.text}</p>
                                        <div class="flex items-center mt-auto">
                                            <img src=b.icon_url alt=b.title width=48 height=48/>
                                            <p class="ml-auto font-code text-xs font-bold text-n-1 uppercase tracking-wider">
                                                "Explore more"
                                            </p>
                                            <Arrow/>
                                        </div>
                                    </div>
                                    <div
                                        class="absolute inset-0 5 bg-n-8"
                                        style=r#"clip-path: url(#benefits)"#
                                    >
                                        <div class="absolute inset-0 opacity-0 transition-opacity hover:opacity-10">
                                            <img
                                                class="w-full h-full object-cover"
                                                src=b.image_url
                                                width=380
                                                height=362
                                                alt=b.title
                                            />
                                        </div>
                                    </div>
                                    <CustomClipPath/>
                                </div>
                            }
                        }
                    })
                    .collect_view()}
            </div>
        </Section>
    }
}
