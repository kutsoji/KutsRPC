use crate::Preset;
use leptos::{
    logging::log,
    *,
};

#[component]
pub fn SideBarIcon<
    F: Fn(ev::MouseEvent) + 'static + Copy,
    E: Fn(ev::MouseEvent) + 'static + Copy,
>(
    preset: Preset,
    i: usize,
    on_click: F,
    on_remove: E,
    current_preset: RwSignal<Option<Preset>>,
) -> impl IntoView {
    let preset_name = store_value(preset.name);
    let class = move || {
        if let Some(p) = current_preset() {
            if p.name == preset_name() {
                "active"
            } else {
                ""
            }
        } else {
            ""
        }
    };
    let show_remove_button = create_rw_signal(false);
    view! {
        <div class="flex">
            <div
                on:click={move |e| {
                    show_remove_button.set(false);
                    on_click(e)
                }}

                on:contextmenu={move |_| {
                    if show_remove_button() {
                        show_remove_button.set(false)
                    } else {
                        show_remove_button.set(true)
                    }
                }}

                class={move || { "sidebaricons bg-dc_nav has-tooltip ".to_owned() + class() }}
            >
                {if let Some(ref picture) = preset.picture {
                    view! {
                        <img
                            class="object-cover"
                            draggable="false"
                            src={format!("https://asset.localhost/{}", picture)}
                            alt={preset_name()}
                        />
                    }
                        .into_view()
                } else {
                    (i + 1).into_view()
                }}

                <span class="tooltip-preset">{preset_name()}</span>
            </div>
            <Show when={move || { show_remove_button() }} fallback={|| ()}>
                <div class="remove-preset-container">
                    <button
                        on:click={on_remove}
                        class="flex items-center border border-dc_white hover:border-[#a8242a] px-2 py-1 transition-colors rounded-md duration-300 hover:bg-[#a8242a] group text-xs"
                    >
                        <svg
                            class="fill-dc_white h-6 w-6"
                            viewBox="0 -0.5 25 25"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                            <g
                                id="SVGRepo_tracerCarrier"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></g>
                            <g id="SVGRepo_iconCarrier">
                                <path
                                    d="M6.96967 16.4697C6.67678 16.7626 6.67678 17.2374 6.96967 17.5303C7.26256 17.8232 7.73744 17.8232 8.03033 17.5303L6.96967 16.4697ZM13.0303 12.5303C13.3232 12.2374 13.3232 11.7626 13.0303 11.4697C12.7374 11.1768 12.2626 11.1768 11.9697 11.4697L13.0303 12.5303ZM11.9697 11.4697C11.6768 11.7626 11.6768 12.2374 11.9697 12.5303C12.2626 12.8232 12.7374 12.8232 13.0303 12.5303L11.9697 11.4697ZM18.0303 7.53033C18.3232 7.23744 18.3232 6.76256 18.0303 6.46967C17.7374 6.17678 17.2626 6.17678 16.9697 6.46967L18.0303 7.53033ZM13.0303 11.4697C12.7374 11.1768 12.2626 11.1768 11.9697 11.4697C11.6768 11.7626 11.6768 12.2374 11.9697 12.5303L13.0303 11.4697ZM16.9697 17.5303C17.2626 17.8232 17.7374 17.8232 18.0303 17.5303C18.3232 17.2374 18.3232 16.7626 18.0303 16.4697L16.9697 17.5303ZM11.9697 12.5303C12.2626 12.8232 12.7374 12.8232 13.0303 12.5303C13.3232 12.2374 13.3232 11.7626 13.0303 11.4697L11.9697 12.5303ZM8.03033 6.46967C7.73744 6.17678 7.26256 6.17678 6.96967 6.46967C6.67678 6.76256 6.67678 7.23744 6.96967 7.53033L8.03033 6.46967ZM8.03033 17.5303L13.0303 12.5303L11.9697 11.4697L6.96967 16.4697L8.03033 17.5303ZM13.0303 12.5303L18.0303 7.53033L16.9697 6.46967L11.9697 11.4697L13.0303 12.5303ZM11.9697 12.5303L16.9697 17.5303L18.0303 16.4697L13.0303 11.4697L11.9697 12.5303ZM13.0303 11.4697L8.03033 6.46967L6.96967 7.53033L11.9697 12.5303L13.0303 11.4697Z"
                                    class
                                ></path>
                            </g>
                        </svg>
                        <span class="text-dc_white mt-0.5 ml-1">Remove Preset</span>
                    </button>
                </div>
            </Show>
        </div>
    }
}
