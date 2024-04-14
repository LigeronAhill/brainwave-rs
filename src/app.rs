use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{
    benefits::Benefits, collaboration::Collaboration, header::Header, hero::Hero,
    svg::ButtonGradient,
};

#[component]
pub fn app() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/svg+xml" href="public/brainwave-symbol.svg"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}
#[component]
fn home() -> impl IntoView {
    view! {
        <div class="pt-[4.75rem] lg:pt-[5.25rem] overflow-hidden">
            <Header/>
            <Hero/>
            <Benefits/>
            <Collaboration/>
        </div>
        <ButtonGradient/>
    }
}
