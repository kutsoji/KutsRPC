use crate::{
    components::main::*,
    invoke,
};
use leptos::{
    logging::log,
    *,
};

#[component]
pub fn CreatePresetModal<
    F: Fn(ev::MouseEvent) + 'static + Copy,
    G: Fn(ev::MouseEvent) + 'static + Copy,
>(
    on_click: F,
    on_cancel: G,
    name: RwSignal<Option<String>>,
    selected_picture: RwSignal<Option<String>>,
    path: RwSignal<Option<String>>,
) -> impl IntoView {
    let select_picture_action = create_action(move |input: &()| async move {
        let result =
            invoke!("open_dialog", {dialogType: String = "File".to_string()}, Option<String>);
        match result {
            Ok(Some(p)) => {
                selected_picture.set(Some(p));
            }
            Err(e) => log!("{}", e.to_string()),
            _ => (),
        }
    });

    view! {
        <div class="relative z-50" aria-labelledby="modal-title" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-dc_nav bg-opacity-75 transition-opacity"></div>

            <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class="relative transform overflow-hidden rounded-lg text-left shadow-xl transition-all sm:my-8">
                        <div class="bg-dc_black px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
                            <div class="sm:flex sm:items-start">
                                <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-dc_green sm:mx-0 sm:h-10 sm:w-10">
                                    <svg
                                        class="h-6 w-6 fill-dc_white"
                                        stroke="currentColor"
                                        aria-hidden="true"
                                        version="1.1"
                                        id="Capa_1"
                                        xmlns="http://www.w3.org/2000/svg"
                                        xmlns:xlink="http://www.w3.org/1999/xlink"
                                        viewBox="0 0 407.096 407.096"
                                        xml:space="preserve"
                                    >
                                        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                                        <g
                                            id="SVGRepo_tracerCarrier"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                        ></g>
                                        <g id="SVGRepo_iconCarrier">
                                            <g>
                                                <g>
                                                    <path d="M402.115,84.008L323.088,4.981C319.899,1.792,315.574,0,311.063,0H17.005C7.613,0,0,7.614,0,17.005v373.086 c0,9.392,7.613,17.005,17.005,17.005h373.086c9.392,0,17.005-7.613,17.005-17.005V96.032 C407.096,91.523,405.305,87.197,402.115,84.008z M300.664,163.567H67.129V38.862h233.535V163.567z"></path>
                                                    <path d="M214.051,148.16h43.08c3.131,0,5.668-2.538,5.668-5.669V59.584c0-3.13-2.537-5.668-5.668-5.668h-43.08 c-3.131,0-5.668,2.538-5.668,5.668v82.907C208.383,145.622,210.92,148.16,214.051,148.16z"></path>
                                                </g>
                                            </g>
                                        </g>
                                    </svg>
                                </div>
                                <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                                    <h3
                                        class="text-base font-semibold leading-6 text-dc_white"
                                        id="modal-title"
                                    >
                                        Preset Info
                                    </h3>
                                    <div class="mt-2 grid grid-cols-2 grid-rows-2">
                                        <div class="row-span-2 space-y-2">
                                            <Input
                                                label="PRESET NAME"
                                                info="set the name for the preset"
                                                main_class="row-span-1"
                                                input_class="input-text"
                                                input_value={MaybeSignal::Dynamic(Signal::from(name))}
                                                on_input={Box::new(move |e| {
                                                    if event_target_value(&e) == "" {
                                                        name.set(None)
                                                    } else {
                                                        name.set(Some(event_target_value(&e)))
                                                    }
                                                })}
                                            />

                                            <Input
                                                label="PRESET DIRECTORY"
                                                info="you can change where your presets are saved in the configuration"
                                                main_class="row-span-1"
                                                input_class="input-text"
                                                disabled=true
                                                input_value={MaybeSignal::Dynamic(path.into_signal())}
                                            />
                                        </div>
                                        <div class="col-span-1 row-span-2 content-center place-content-center">
                                            <div class="relative flex justify-center items-center group">
                                                <img
                                                    draggable="false"
                                                    prop:src={move || {
                                                        selected_picture()
                                                            .and_then(|p| Some(
                                                                format!("https://asset.localhost/{}", p),
                                                            ))
                                                            .or(Some("../../assets/imgs/test.png".to_string()))
                                                    }}

                                                    alt="Avatar"
                                                    class="rounded-full border-solid border-4 border-dc_gray absolute flex justify-center items-center h-[128px] w-[128px] object-cover"
                                                />
                                                <button
                                                    on:click={move |e| select_picture_action.dispatch(())}
                                                    class="h-10 w-24 z-10 text-dc_gray border-2 border-dc_gray text-xs outline-none rounded-md transition-colors duration-300 absolute flex justify-center items-center group-hover:bg-dc_gray group-hover:text-dc_black"
                                                >
                                                    Select Picture
                                                </button>
                                            </div>
                                        </div>

                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="bg-dc_nav px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                            <button
                                on:click={move |e| {
                                    on_click(e);
                                }}

                                type="button"
                                class="inline-flex w-6/12 justify-center rounded-md transition-colors duration-200 bg-dc_green hover:bg-[#5baf77] px-3 py-2 text-sm font-semibold text-dc_white shadow-sm sm:ml-3 sm:w-auto"
                            >
                                Create Preset
                            </button>
                            <button
                                on:click={move |e| { on_cancel(e) }}

                                type="button"
                                class="inline-flex w-6/12 justify-center rounded-md transition-colors duration-200 bg-dc_btn outline-none hover:bg-[#676A75] px-3 py-2 text-sm font-semibold text-dc_white shadow-sm sm:ml-3 sm:w-auto"
                            >
                                Cancel
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
