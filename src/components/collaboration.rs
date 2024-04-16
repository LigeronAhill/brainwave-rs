use leptos::*;

use crate::{
    components::{button::Button, section::Section},
    constants::{COLLABTEXT, COLLAB_APPS, COLLAB_CONTENT},
};

#[component]
pub fn Collaboration() -> impl IntoView {
    view! {
        <Section crosses=true>
            <div class="container lg:flex">
                <div class="max-w-[25rem]">
                    <h2 class="h2 mb-4 md:mb-8">"AI Chat App for seamless collaboration"</h2>
                    <ul class="max-w-[22rem] mb-10 md:mb-14">
                        {COLLAB_CONTENT
                            .into_iter()
                            .map(|item| {
                                if item.text.is_empty() {
                                    view! {
                                        <li class="mb-3 py-3" key="{item.id}">
                                            <div class="flex items-center">
                                                <img
                                                    src="public/check.svg"
                                                    width=24
                                                    height=24
                                                    alt="check"
                                                />
                                                <h6 class="body-2 ml-5">{item.title}</h6>
                                            </div>
                                        </li>
                                    }
                                } else {
                                    view! {
                                        <li class="mb-3 py-3" key="{item.id}">
                                            <div class="flex items-center">
                                                <img
                                                    src="public/check.svg"
                                                    width=24
                                                    height=24
                                                    alt="check"
                                                />
                                                <h6 class="body-2 ml-5">{item.title}</h6>
                                            </div>
                                            <p class="body-2 mt-3 text-n-4">{item.text}</p>
                                        </li>
                                    }
                                }
                            })
                            .collect_view()}
                    </ul>
                    <Button>"Try it now"</Button>
                </div>
                <div class="lg:ml-auto xl:w-[38rem] mt-4">
                    <p class="body-2 mb-4 text-n-4 md:mb-16 lg:mb-32 lg:w-[22rem] lg:mx-auto">
                        {COLLABTEXT}
                    </p>
                    <div class="relative left-1/2 flex w-[22rem] aspect-square border border-n-6 rounded-full -translate-x-1/2 scale:75 md:scale-100">
                        <div class="flex w-60 aspect-square m-auto border border-n-6 rounded-full">
                            <div class="w-[6rem] aspect-square m-auto p-[0.2rem] bg-conic-gradient rounded-full">
                                <div class="flex items-center justify-center w-full h-full bg-n-8 rounded-full">
                                    <img
                                        src="public/brainwave-symbol.svg"
                                        alt="brainwave"
                                        width=48
                                        height=48
                                    />

                                </div>
                            </div>
                        </div>
                        <ul>
                            {COLLAB_APPS
                                .into_iter()
                                .enumerate()
                                .map(|(index, app)| {
                                    view! {
                                        <li
                                            key=app.id
                                            class=format!(
                                                "absolute top-0 left-1/2 h-1/2 -ml-[1.6rem] origin-bottom rotate-{}",
                                                index * 45,
                                            )
                                        >

                                            <div class=format!(
                                                "relative -top-[1.6rem] flex w-[3.2rem] h-[3.2rem] bg-n-7 border border-n-1/15 rounded-xl -rotate-{}",
                                                index * 45,
                                            )>
                                                <img
                                                    class="m-auto"
                                                    width=app.width
                                                    height=app.height
                                                    src=app.icon
                                                    alt=app.title
                                                />
                                            </div>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                        <div class="hidden absolute top-1/2 right-full w-[32.625rem] -mt-1 mr-10 pointer-events-none xl:block">
                            <img
                                src="public/collaboration/curve-1.svg"
                                alt="Curve 1"
                                width=522
                                height=182
                            />
                        </div>
                        <div class="hidden absolute top-1/2 left-full w-[10.125rem] -mt-1 ml-10 pointer-events-none xl:block">
                            <img
                                src="public/collaboration/curve-2.svg"
                                alt="Curve 2"
                                width=162
                                height=76
                            />
                        </div>
                    </div>
                </div>
            </div>
        </Section>
    }
}
