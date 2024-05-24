use leptos::*;

#[component]
pub fn ErrorModal<F: Fn(ev::MouseEvent) + 'static + Copy>(
    #[prop(optional)] title: String,
    #[prop(optional)] description: String,
    button_title: String,
    on_click: F,
) -> impl IntoView {
    let (open, set_open) = create_signal(true);
    view! {
        <Show when={move || open()} fallback={|| ()}>
            <div
                class="relative z-20"
                aria-labelledby="modal-title"
                role="dialog"
                aria-modal="true"
            >
                <div class="fixed inset-0 bg-dc_nav bg-opacity-75 transition-opacity"></div>

                <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                    <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                        <div class="relative transform overflow-hidden rounded-lg text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
                            <div class="bg-dc_black px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                                <div class="sm:flex sm:items-start">
                                    <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-[#a8242a] sm:mx-0 sm:h-10 sm:w-10">
                                        <svg
                                            class="h-6 w-6"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            aria-hidden="true"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
                                            ></path>
                                        </svg>
                                    </div>
                                    <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                                        <h3
                                            class="text-base font-semibold leading-6 text-dc_white"
                                            id="modal-title"
                                        >
                                            {title.clone()}
                                        </h3>
                                        <div class="mt-2">
                                            <p class="text-sm text-dc_white whitespace-pre-wrap">
                                                {description.clone()}
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="bg-dc_nav px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                                <button
                                    on:click={move |e| {
                                        set_open(false);
                                        on_click(e)
                                    }}

                                    type="button"
                                    class="inline-flex w-full justify-center rounded-md transition-colors duration-200 bg-[#a8242a] hover:bg-[#c53d3d] px-3 py-2 text-sm font-semibold text-dc_white shadow-sm sm:ml-3 sm:w-auto"
                                >
                                    {button_title.clone()}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

        </Show>
    }
}
