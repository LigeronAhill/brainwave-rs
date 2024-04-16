use leptos::*;

use crate::components::{
    button::Button,
    company_logos::CompanyLogos,
    design::hero::{BackgroundCircles, BottomLine, Gradient},
    generating::Generating,
    notification::Notification,
    section::Section,
};
use crate::constants::HERO_ICONS;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <Section
            class_name="pt-[12rem] -mt-[5.25rem]"
            crosses=true
            crosses_offset="lg:translate-y-[5.25rem]"
            custom_paddings=true
            id="hero"
        >
            <div class="container relative">
                <div class="relative z-1 max-w-[62rem] mx-auto text-center mb-[3.875rem] md:mb-20 lg:mb-[6.25rem]">
                    <h1 class="h1 mb-6">
                        "Explore the Possibilities of AI Chatting with "
                        <span class="inline-block relative">
                            "Brainwave"
                            <img
                                src="public/hero/curve.png"
                                class="absolute top-full left-0 w-full xl:-mt-2"
                                width=624
                                height=28
                                alt="Curve"
                            />
                        </span>
                    </h1>
                    <p class="body-1 max-w-3xl mx-auto mb-6 text-n-2 lg:mb-8">
                        "Unleash the power of AI within Brainwave. Upgrade your productivity with Brainwave, the open AI chat app."
                    </p>
                    <Button href="/pricing".to_string() white=true>
                        "Get started"
                    </Button>
                </div>
                <div class="relative max-w-[23rem] mx-auto md:max-w-5xl xl:mb-24">
                    <div class="relative z-1 p-0.5 rounded-2xl bg-conic-gradient">
                        <div class="relative bg-n-8 rounded-[1rem]">
                            <div class="h-[1.4rem] bg-n-10 rounded-t-[0.9rem]"></div>

                            <div class="aspect-[33/40] rounded-b-[0.9rem] overflow-hidden md:aspect-[688/490] lg:aspect-[1024/490]">
                                <img
                                    src="public/hero/robot.jpg"
                                    className="w-full scale-[1.7] translate-y-[8%] md:scale-[1] md:-translate-y-[10%] lg:-translate-y-[23%]"
                                    width=1024
                                    height=490
                                    alt="AI"
                                />

                                <Generating class="absolute left-4 right-4 bottom-5 md:left-1/2 md:right-auto md:bottom-8 md:w-[31rem] md:-translate-x-1/2"/>

                                <ul class="hidden absolute -left-[5.5rem] bottom-[7.5rem] px-1 py-1 bg-n-9/40 backdrop-blur border border-n-1/10 rounded-2xl xl:flex">
                                    {HERO_ICONS
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, icon)| {
                                            view! {
                                                <li class="p-5" key=index>
                                                    <img src=icon width=24 height=25 alt=icon/>
                                                </li>
                                            }
                                        })
                                        .collect_view()}

                                </ul>

                                <Notification
                                    class="hidden absolute -right-[5.5rem] bottom-[11rem] w-[18rem] xl:flex"
                                    title="Code generation"
                                />
                            </div>
                        </div>

                        <Gradient/>
                    </div>
                    <div class="absolute -top-[54%] left-1/2 w-[234%] -translate-x-1/2 md:-top-[46%] md:w-[138%] lg:-top-[104%]">
                        <img
                            src="public/hero/hero-background.jpg"
                            class="w-full"
                            width=1440
                            height=1800
                            alt="hero"
                        />
                    </div>

                    <BackgroundCircles/>
                </div>

                <CompanyLogos class="hidden relative z-10 mt-20 lg:block"/>
            </div>

            <BottomLine/>
        </Section>
    }
}
