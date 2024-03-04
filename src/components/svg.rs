use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn ButtonGradient() -> impl IntoView {
    view! {
        <svg class="block" width="0" height="0">
            <defs>
                <linearGradient id="btn-left" x1="50%" x2="50%" y1="0%" y2="100%">
                    <stop offset="0%" stop-color="#89F9E8"></stop>
                    <stop offset="100%" stop-color="#FACB7B"></stop>
                </linearGradient>
                <linearGradient id="btn-top" x1="100%" x2="0%" y1="50%" y2="50%">
                    <stop offset="0%" stop-color="#D87CEE"></stop>
                    <stop offset="100%" stop-color="#FACB7B"></stop>
                </linearGradient>
                <linearGradient id="btn-bottom" x1="100%" x2="0%" y1="50%" y2="50%">
                    <stop offset="0%" stop-color="#9099FC"></stop>
                    <stop offset="100%" stop-color="#89F9E8"></stop>
                </linearGradient>
                <linearGradient id="btn-right" x1="14.635%" x2="14.635%" y1="0%" y2="100%">
                    <stop offset="0%" stop-color="#9099FC"></stop>
                    <stop offset="100%" stop-color="#D87CEE"></stop>
                </linearGradient>
            </defs>
        </svg>
    }
}
#[allow(non_snake_case)]
#[component]
pub fn ButtonSvg(white: bool) -> impl IntoView {
    view! {
        <svg class="absolute top-0 left-0" width="21" height="44" viewBox="0 0 21 44">
            <path
                fill=(if white { "white" } else { "none" })
                stroke=(if white { "white" } else { "url(#btn-left)" })
                strokeWidth="2"
                d="M21,43.00005 L8.11111,43.00005 C4.18375,43.00005 1,39.58105 1,35.36365 L1,8.63637 C1,4.41892 4.18375,1 8.11111,1 L21,1"
            ></path>
        </svg>
        <svg
            class="absolute top-0 left-[1.3125rem] w-[calc(100%-2.625rem)]"
            height="44"
            viewBox="0 0 100 44"
            preserveAspectRatio="none"
            fill=(if white { "white" } else { "none" })
        >
            {if white {
                view! {
                    <polygon
                        fill="white"
                        fillRule="nonzero"
                        points="100 0 100 44 0 44 0 0"
                    ></polygon>
                }
                    .into_view()
            } else {
                view! {
                    <polygon
                        // mentioned in ButtonGradient.jsx
                        fill="url(#btn-top)"
                        fillRule="nonzero"
                        points="100 42 100 44 0 44 0 42"
                    ></polygon>
                    <polygon
                        // mentioned in ButtonGradient.jsx
                        fill="url(#btn-bottom)"
                        fillRule="nonzero"
                        points="100 0 100 2 0 2 0 0"
                    ></polygon>
                }
                    .into_view()
            }}

        </svg>
        <svg class="absolute top-0 right-0" width="21" height="44" viewBox="0 0 21 44">
            <path
                fill=(if white { "white" } else { "none" })
                // mentioned in ButtonGradient.jsx
                stroke=(if white { "white" } else { "url(#btn-right)" })
                strokeWidth="2"
                d="M0,43.00005 L5.028,43.00005 L12.24,43.00005 C16.526,43.00005 20,39.58105 20,35.36365 L20,16.85855 C20,14.59295 18.978,12.44425 17.209,10.99335 L7.187,2.77111 C5.792,1.62675 4.034,1 2.217,1 L0,1"
            ></path>
        </svg>
    }
}
#[allow(non_snake_case)]
#[component]
pub fn MenuSvg(open_navigation: ReadSignal<bool>) -> impl IntoView {
    view! {
        <svg class="overflow-visible" width="20" height="12" viewBox="0 0 20 12">
            <rect
                class="transition-all origin-center"
                y=(move || if open_navigation() { "5" } else { "0" })
                width="20"
                height="2"
                rx="1"
                fill="white"
                transform=(move || if open_navigation() { "rotate(45)" } else { "rotate(0)" })
            ></rect>
            <rect
                class="transition-all origin-center"
                y=(move || if open_navigation() { "5" } else { "10" })
                width="20"
                height="2"
                rx="1"
                fill="white"
                transform=(move || if open_navigation() { "rotate(-45)" } else { "rotate(0)" })
            ></rect>
        </svg>
    }
}
