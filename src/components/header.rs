use leptos::*;
use leptos_router::use_location;

use crate::{
    components::{button::Button, design::hamburger::HamburgerMenu, svg::MenuSvg},
    constants::NavLink,
};

#[allow(non_snake_case)]
#[component]
pub fn Header() -> impl IntoView {
    let navigations = NavLink::iter();
    let path_name = use_location();
    let (open_navigation, set_open_navigation) = create_signal(false);
    view! {
        <div class=(move || {
            if open_navigation() {
                "fixed top-0 left-0 w-full z-50  border-b border-n-6 lg:bg-n-8/90 lg:backdrop-blur-sm bg-n-8"
            } else {
                "fixed top-0 left-0 w-full z-50  border-b border-n-6 lg:bg-n-8/90 lg:backdrop-blur-sm bg-n-8/90 backdrop-blur-sm"
            }
        })>
            <div class="flex items-center px-5 lg:px-7.5 xl:px-10 max-lg:py-4">
                <a href="#hero" class="block w-[12rem] xl:mr-8">
                    <img src="public/brainwave.svg" alt="Brainwave"/>
                </a>

                <nav class=(move || {
                    if open_navigation() {
                        "fixed top-[5rem] left-0 right-0 bottom-0 bg-n-8 lg:static lg:flex lg:mx-auto lg:bg-transparent flex"
                    } else {
                        "fixed top-[5rem] left-0 right-0 bottom-0 bg-n-8 lg:static lg:flex lg:mx-auto lg:bg-transparent hidden"
                    }
                })>
                    <div class="relative z-2 flex flex-col items-center justify-center m-auto lg:flex-row">
                        {navigations
                            .into_iter()
                            .map(|n| {
                                let mobile_class = if n.only_mobile { "lg:hidden" } else { "" };
                                let url_class = move || {
                                    if *n.url == path_name.hash.get_untracked() {
                                        "z-2 lg:text-n-1"
                                    } else {
                                        "lg:text-n-1/50"
                                    }
                                };
                                let class = format!(
                                    "block relative font-code text-2xl uppercase text-n-1 transition-colors hover:text-color-1 {mobile_class} px-6 py-6 md:py-8 lg:-mr-0.25 lg:text-xs lg:font-semibold {} lg:leading-5 lg:hover:text-n-1 xl:px-12",
                                    url_class(),
                                );
                                view! {
                                    <a
                                        href=n.url
                                        key=n.id
                                        on:click=move |_| set_open_navigation(false)
                                        class=class
                                    >
                                        {n.title}
                                    </a>
                                }
                            })
                            .collect_view()}

                    </div>
                    <HamburgerMenu/>
                </nav>
                <a
                    href="#signup"
                    class="button hidden mr-8 text-n-1/50 transition-colors hover:text-n-1 lg:block"
                >
                    New Account
                </a>
                <Button class="hidden lg:flex".to_string() href="#login".to_string()>
                    {"Sign in"}
                </Button>
                <Button
                    class="ml-auto lg:hidden".to_string()
                    px="px-3".to_string()
                    on:click=move |_| set_open_navigation.update(|n| *n = !*n)
                >
                    <MenuSvg open_navigation=open_navigation/>
                </Button>
            </div>
        </div>
    }
}
