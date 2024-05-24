use std::path::{
    Path,
    PathBuf,
};

use crate::{
    components::main::*,
    invoke,
    Config,
    GlobalState,
};
use leptos::{
    logging::log,
    MaybeSignal::Static,
    *,
};

#[component]
pub fn ConfigModal<
    F: Fn(ev::MouseEvent) + 'static + Copy,
    G: Fn(ev::MouseEvent) + 'static + Copy,
>(
    on_click: F,
    on_cancel: G,
    app_id: RwSignal<Option<String>>,
    remember: RwSignal<bool>,
    presets_dir: RwSignal<Option<String>>,
    minimize: RwSignal<bool>,
) -> impl IntoView {
    let set_directory_action = create_action(move |input: &()| async move {
        let result =
            invoke!("open_dialog", {dialogType: String = "Folder".to_string()}, Option<String>);
        match result {
            Ok(Some(p)) => {
                presets_dir.set(Some(p));
            }
            Err(e) => log!("{}", e.to_string()),
            _ => (),
        }
    });
    view! {
        <div class="relative z-30" aria-labelledby="modal-title" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-dc_nav bg-opacity-75 transition-opacity"></div>

            <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class="relative transform text-left shadow-xl transition-all sm:my-8">
                        <div class="bg-dc_black px-4 pb-4 pt-5 sm:p-6 sm:pb-4 rounded-t-lg">
                            <div class="sm:flex sm:items-start">
                                <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-dc_green sm:mx-0 sm:h-10 sm:w-10">
                                    <svg
                                        class="h-6 w-6 fill-dc_white fill-current flex-shrink-0"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            xmlns="http://www.w3.org/2000/svg"
                                            d="M 10.490234 2 C 10.011234 2 9.6017656 2.3385938 9.5097656 2.8085938 L 9.1757812 4.5234375 C 8.3550224 4.8338012 7.5961042 5.2674041 6.9296875 5.8144531 L 5.2851562 5.2480469 C 4.8321563 5.0920469 4.33375 5.2793594 4.09375 5.6933594 L 2.5859375 8.3066406 C 2.3469375 8.7216406 2.4339219 9.2485 2.7949219 9.5625 L 4.1132812 10.708984 C 4.0447181 11.130337 4 11.559284 4 12 C 4 12.440716 4.0447181 12.869663 4.1132812 13.291016 L 2.7949219 14.4375 C 2.4339219 14.7515 2.3469375 15.278359 2.5859375 15.693359 L 4.09375 18.306641 C 4.33275 18.721641 4.8321562 18.908906 5.2851562 18.753906 L 6.9296875 18.1875 C 7.5958842 18.734206 8.3553934 19.166339 9.1757812 19.476562 L 9.5097656 21.191406 C 9.6017656 21.661406 10.011234 22 10.490234 22 L 13.509766 22 C 13.988766 22 14.398234 21.661406 14.490234 21.191406 L 14.824219 19.476562 C 15.644978 19.166199 16.403896 18.732596 17.070312 18.185547 L 18.714844 18.751953 C 19.167844 18.907953 19.66625 18.721641 19.90625 18.306641 L 21.414062 15.691406 C 21.653063 15.276406 21.566078 14.7515 21.205078 14.4375 L 19.886719 13.291016 C 19.955282 12.869663 20 12.440716 20 12 C 20 11.559284 19.955282 11.130337 19.886719 10.708984 L 21.205078 9.5625 C 21.566078 9.2485 21.653063 8.7216406 21.414062 8.3066406 L 19.90625 5.6933594 C 19.66725 5.2783594 19.167844 5.0910937 18.714844 5.2460938 L 17.070312 5.8125 C 16.404116 5.2657937 15.644607 4.8336609 14.824219 4.5234375 L 14.490234 2.8085938 C 14.398234 2.3385937 13.988766 2 13.509766 2 L 10.490234 2 z M 12 8 C 14.209 8 16 9.791 16 12 C 16 14.209 14.209 16 12 16 C 9.791 16 8 14.209 8 12 C 8 9.791 9.791 8 12 8 z"
                                        ></path>
                                    </svg>
                                </div>
                                <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                                    <h3
                                        class="text-base font-semibold leading-6 text-dc_white"
                                        id="modal-title"
                                    >
                                        Configuration
                                    </h3>
                                    <div class="mt-2 gap-y-1.5 gap-x-4 grid grid-cols-4 grid-rows-2">
                                        <Input
                                            label="APPLICATION ID"
                                            info="set the Bot Application ID to be used for your rich presence"
                                            input_class="input-text"
                                            main_class="col-span-3"
                                            on_input={Box::new(move |e| {
                                                if event_target_value(&e) == "" {
                                                    app_id.set(None)
                                                } else {
                                                    app_id.set(Some(event_target_value(&e)))
                                                }
                                            })}

                                            input_value={MaybeSignal::Dynamic(Signal::from(app_id))}
                                        />
                                        <Input
                                            input_id="remember_checkbox"
                                            label="REMEMBER"
                                            info="set whether you want the Bot Application ID to be saved"
                                            input_type="checkbox"
                                            input_class="input-checkbox group"

                                            on_input={Box::new(move |e| {
                                                remember.set(event_target_checked(&e))
                                            })}

                                            checked={remember()}
                                            extra_html="
                                            <label for='remember_checkbox' class='cursor-pointer absolute left-[3px] -top-[30px]'>
                                            <svg class='w-[18px] h-[18px] hidden fill-none' viewBox='0 -0.5 25 25' xmlns='http://www.w3.org/2000/svg'>
                                            <g id='SVGRepo_bgCarrier' stroke-width='0'></g>
                                            <g id='SVGRepo_tracerCarrier' stroke-linecap='round' stroke-linejoin='round'></g>
                                            <g id='SVGRepo_iconCarrier'>
                                            <path d='M5.5 12.5L10.167 17L19.5 8' stroke='#000000' stroke-width='1.5' stroke-linecap='round'
                                            stroke-linejoin='round'></path>
                                            </g>
                                            </svg>
                                            </label>"
                                        />
                                        <Input
                                            label="PRESETS DIRECTORY"
                                            info="set the directory where presets are saved"
                                            input_class="input-text"
                                            main_class="col-span-3"
                                            on_input={Box::new(move |e| {
                                                set_directory_action.dispatch(());
                                                if event_target_value(&e) == "" {
                                                    presets_dir.set(None)
                                                } else {
                                                    presets_dir.set(Some(event_target_value(&e)))
                                                }
                                            })}

                                            input_value={MaybeSignal::Dynamic(
                                                Signal::from(presets_dir),
                                            )}
                                        />

                                        <Input
                                            input_id="minimize_checkbox"
                                            label="MINIMIZE"
                                            info="set whether you want the app to go into system tray after closing it"
                                            input_type="checkbox"
                                            input_class="input-checkbox"
                                            on_input={Box::new(move |e| {
                                                minimize.set(event_target_checked(&e))
                                            })}

                                            checked={minimize()}
                                            extra_html="
                                            <label for='minimize_checkbox' class='cursor-pointer absolute left-[3px] -top-[30px]'>
                                            <svg class='w-[18px] h-[18px] hidden fill-none' viewBox='0 -0.5 25 25' xmlns='http://www.w3.org/2000/svg'>
                                            <g id='SVGRepo_bgCarrier' stroke-width='0'></g>
                                            <g id='SVGRepo_tracerCarrier' stroke-linecap='round' stroke-linejoin='round'></g>
                                            <g id='SVGRepo_iconCarrier'>
                                            <path d='M5.5 12.5L10.167 17L19.5 8' stroke='#000000' stroke-width='1.5' stroke-linecap='round'
                                            stroke-linejoin='round'></path>
                                            </g>
                                            </svg>
                                            </label>"
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="bg-dc_nav px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6 rounded-b-lg">
                            <button
                                on:click={move |e| {
                                    on_click(e);
                                }}

                                class="inline-flex w-6/12 justify-center rounded-md transition-colors duration-200 bg-dc_green hover:bg-[#5baf77] px-3 py-2 text-sm font-semibold text-dc_white shadow-sm sm:ml-3 sm:w-auto"
                            >
                                OK
                            </button>
                            <button
                                on:click={move |e| { on_cancel(e) }}
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
