use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{
    benefits::Benefits, collaboration::Collaboration, footer::Footer, header::Header, hero::Hero,
    pricing::Pricing, roadmap::Roadmap, services::Services, svg::ButtonGradient,
};

#[component]
pub fn App() -> impl IntoView {
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
fn Home() -> impl IntoView {
    view! {
        <div class="pt-20 overflow-hidden">
            <Header/>
            <Hero/>
            <Benefits/>
            <Collaboration/>
            <Services/>
            <Pricing/>
            <Roadmap/>
            <Footer/>
        </div>
        <ButtonGradient/>
    }
}