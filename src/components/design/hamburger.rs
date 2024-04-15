use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn Rings() -> impl IntoView {
    view! {
        <div class="absolute top-1/2 left-1/2 w-[51.375rem] aspect-square border border-n-2/10 rounded-full -translate-x-1/2 -translate-y-1/2">
            <div class="absolute top-1/2 left-1/2 w-[36.125rem] aspect-square border border-n-2/10 rounded-full -translate-x-1/2 -translate-y-1/2"></div>
            <div class="absolute top-1/2 left-1/2 w-[23.125rem] aspect-square border border-n-2/10 rounded-full -translate-x-1/2 -translate-y-1/2"></div>
        </div>
    }
}
#[allow(non_snake_case)]
#[component]
pub fn SideLines() -> impl IntoView {
    view! {
        <div class="absolute top-0 left-5 w-0.25 h-full bg-n-6"></div>
        <div class="absolute top-0 right-5 w-0.25 h-full bg-n-6"></div>
    }
}
#[allow(non_snake_case)]
#[component]
pub fn BackgroundCircles() -> impl IntoView {
    view! {
        <div class="absolute top-[4.4rem] left-16 w-3 h-3 bg-gradient-to-b from-[#DD734F] to-[#1A1A32] rounded-full"></div>
        <div class="absolute top-[12.6rem] right-16 w-3 h-3 bg-gradient-to-b from-[#B9AEDF] to-[#1A1A32] rounded-full"></div>
        <div class="absolute top-[26.8rem] left-12 w-6 h-6 bg-gradient-to-b from-[#88E5BE] to-[#1A1A32] rounded-full"></div>
    }
}
#[allow(non_snake_case)]
#[component]
pub fn HamburgerMenu() -> impl IntoView {
    view! {
        <div class="absolute inset-0 pointer-events-none lg:hidden">
            <div class="absolute inset-0 opacity-[.03]">
                <img
                    class="w-full h-full object-cover"
                    src="/public/background.jpg"
                    width=688
                    height=953
                    alt="Background"
                />
            </div>

            <Rings/>

            <SideLines/>

            <BackgroundCircles/>
        </div>
    }
}