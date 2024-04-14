use leptos::*;

use crate::{
    components::{generating::Generating, heading::Heading, section::Section},
    constants::{BRAINWAVE_SERVICES, BRAINWAVE_SERVICES_ICONS},
};

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <Section id="how-to-use">
            <div class="container">
                <Heading
                    title="Generative AI made for creators"
                    text="Brainwave unlocks the potential of AI-powered applications"
                />
                <div class="relative">
                    <div class="relative z-1 flex items-center h-[39rem] mb-5 p-8 border border-n-1/10 rounded-3xl overfow-hidden lg:p-20 xl:h-[46rem]">
                        <div class="absolute top-0 left-0 w-full h-full pointer-events-none md:w-3/5 xl:w-auto">
                            <img
                                src="/public/services/service-1.png"
                                alt="Smartest AI"
                                class="w-full h-full object-cover md:object-right"
                                width=800
                            />
                        </div>
                        <div class="relative z-1 max-w-[17rem] ml-auto">
                            <h4 class="h4 mb-4">"Smartest AI"</h4>
                            <p class="body-2 mb-[3rem] text-n-3">
                                "Brainwave unlocks the potencial of AI powered applications"
                            </p>
                            <ul class="body-2">
                                {BRAINWAVE_SERVICES
                                    .into_iter()
                                    .map(|s| {
                                        view! {
                                            <li key=s class="flex items-start py-6 border-t border-n-6">
                                                <img
                                                    src="/public/check.svg"
                                                    alt="check"
                                                    width=24
                                                    height=24
                                                />
                                                <p class="ml-4">{s}</p>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        </div>
                        <Generating class="absolute left-4 right-4 bottom-4 border border-n-1/10 lg:left-1/2 lg:rigth-auto lg:bottom-8 lg:-translate-x-1/2"/>
                    </div>

                    <div class="relative z-1 grid gap-5 lg:grid-cols-2">
                        <div class="relative min-h-[39rem] border border-n-1/10 rounded-3xl overflow-hidden">
                            <div class="absolute inset-0">
                                <img
                                    src="/public/services/service-2.png"
                                    alt="robot"
                                    class="h-full w-full object-cover"
                                    width=630
                                    height=750
                                />
                            </div>
                            <div class="absolute inset-0 flex flex-col justify-end p-8 bg-gradient-to-b from-n-8/0 to-n-8/90 lg:p-15">
                                <h4 class="h4 mb-4">"Photo editing"</h4>
                                <p class="body-2 mb-[3rem] text-n-3">
                                    "Automatically enhance your photos using our AI app's photo editing feature. Try it now!"
                                </p>
                            </div>
                            <PhotoChatMessage/>
                        </div>

                        <div class="p-4 bg-n-7 rounded-3xl overflow-hidden lg:min-h-[46rem]">
                            <div class="py-12 px-4 xl:px-8">
                                <h4 class="h4 mb-4">"Video generation"</h4>
                                <p class="body-2 mb-[2rem] text-n-3">
                                    "The world's most powerfull AI photo and video art generation engine. What will you create?"
                                </p>
                                <ul class="flex items-center justify-between">
                                    {BRAINWAVE_SERVICES_ICONS
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, icon)| {
                                            let index_class = if index == 2 {
                                                "w-[3rem] h-[3rem] p-0.25 bg-conic-gradient md:w-[4.5rem] md:h-[4.5rem]"
                                            } else {
                                                "flex w-10 h-10 bg-n-6 md:w-15 md:h-15"
                                            };
                                            let div_class = if index == 2 {
                                                "flex items-center justify-center w-full h-full bg-n-7 rounded-[1rem]"
                                            } else {
                                                ""
                                            };
                                            view! {
                                                <li
                                                    key=index
                                                    class=format!(
                                                        "rounded-2xl flex items-center justify-center {index_class}",
                                                    )
                                                >
                                                    <div class=div_class>
                                                        <img src=icon alt=icon width=24 height=24/>
                                                    </div>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </div>

                            <div class="relative h-[20rem] bg-n-8 rounded-xl overflow-hidden md:h-[25rem]">
                                <img
                                    class="w-full h-full object-cover"
                                    src="/public/services/service-3.png"
                                    alt="Scary robot"
                                    width=520
                                    height=400
                                />
                                <VideoChatMessage/>
                                <VideoBar/>
                            </div>

                        </div>
                    </div>
                    <Gradient/>
                </div>
            </div>
        </Section>
    }
}

#[component]
pub fn ChatBubbleWing(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] path_class: String,
) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="26" height="37">
            <path
                class=path_class
                d="M21.843 37.001c3.564 0 5.348-4.309 2.829-6.828L3.515 9.015A12 12 0 0 1 0 .53v36.471h21.843z"
            ></path>
        </svg>
    }
}
#[component]
pub fn PhotoChatMessage() -> impl IntoView {
    view! {
        <div class="absolute top-8 right-8 max-w-[17.5rem] py-6 px-8 bg-black rounded-t-xl rounded-bl-xl font-code text-base lg:top-16 lg:right-[8.75rem] lg:max-w-[17.5rem]">
            "Hey Brainwave, enhance this photo"
            <ChatBubbleWing class="absolute left-full bottom-0"/>
        </div>
    }
}
#[component]
pub fn Gradient() -> impl IntoView {
    view! {
        <div class="absolute top-0 -left-[10rem] w-[56.625rem] h-[56.625rem] opacity-50 mix-blend-color-dodge pointer-events-none">
            <img
                class="absolute top-1/2 left-1/2 w-[79.5625rem] max-w-[79.5625rem] h-[88.5625rem] -translate-x-1/2 -translate-y-1/2"
                src="/public/gradient.png"
                width=1417
                height=1417
                alt="Gradient"
            />
        </div>
    }
}
#[component]
pub fn VideoChatMessage() -> impl IntoView {
    view! {
        <div class="absolute top-8 left-[3.125rem] w-full max-w-[14rem] pt-2.5 pr-2.5 pb-7 pl-5 bg-n-6 rounded-t-xl rounded-br-xl font-code text-base md:max-w-[17.5rem]">
            "Video generated!"
            <div class="absolute left-5 -bottom-[1.125rem] flex items-center justify-center w-[2.25rem] h-[2.25rem] bg-color-1 rounded-[0.75rem]">
                <img src="/public/brainwave-symbol-white.svg" width=26 height=26 alt="Brainwave"/>
            </div>
            <p class="tagline absolute right-2.5 bottom-1 text-[0.625rem] text-n-3 uppercase">
                "just now"
            </p>
            <ChatBubbleWing
                class="absolute right-full bottom-0 -scale-x-100"
                path_class="fill-n-6"
            />
        </div>
    }
}
#[component]
pub fn VideoBar() -> impl IntoView {
    view! {
        <div class="absolute left-0 bottom-0 w-full flex items-center p-6">
            <img src="/public/play.svg" width=24 height=24 alt="Play" class="object-contain mr-3"/>

            <div class="flex-1 bg-[#D9D9D9]">
                <div class="w-1/2 h-0.5 bg-color-1"></div>
            </div>
        </div>
    }
}
