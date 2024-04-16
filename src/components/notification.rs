use leptos::*;

use crate::constants::NOTIFICATION_IMAGES;

#[component]
pub fn Notification(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] title: String,
) -> impl IntoView {
    view! {
        <div class=format!(
            "{class} flex items-center p-4 pr-6 bg-n-9/40 backdrop-blur border border-n-1/10 rounded-2xl gap-5",
        )>
            <img
                src="public/notification/image-1.png"
                width=62
                height=62
                alt="image"
                className="rounded-xl"
            />

            <div class="flex-1">
                <h6 class="mb-1 font-semibold text-base">{title}</h6>

                <div class="flex items-center justify-between">
                    <ul class="flex -m-0.5">
                        {NOTIFICATION_IMAGES
                            .into_iter()
                            .enumerate()
                            .map(|(index, item)| {
                                view! {
                                    <li
                                        key=index
                                        class="flex w-6 h-6 border-2 border-n-12 rounded-full overflow-hidden"
                                    >
                                        <img src=item class="w-full" width=20 height=20 alt=item/>
                                    </li>
                                }
                            })
                            .collect_view()}

                    </ul>
                    <div class="body-2 text-n-13">"1m ago"</div>
                </div>
            </div>
        </div>
    }
}
