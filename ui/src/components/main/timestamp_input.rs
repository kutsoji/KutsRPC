use leptos::*;
#[component]
pub fn TimestampInput<F: Fn(ev::Event) + Clone + 'static>(
    label: &'static str,
    info: &'static str,
    selected_option: RwSignal<Option<String>>,
    on_select: F,
) -> impl IntoView {
    let custom_time: RwSignal<Option<String>> = create_rw_signal(None);
    view! {
        <div class="space-y-1 text-xs col-span-2">
            <div class="flex items-center space-x-2">
                <span class="font-semibold text-dc_gray mt-1">{label}</span>
                <div class="has-tooltip icon">
                    <svg
                        class="fill-dc_gray"
                        xmlns="http://www.w3.org/2000/svg"
                        xmlns:xlink="http://www.w3.org/1999/xlink"
                        width="14px"
                        height="14px"
                        viewBox="0 0 93.936 93.936"
                        xml:space="preserve"
                    >
                        <g stroke-width="0"></g>
                        <g
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke="#CCCCCC"
                            stroke-width="2.81808"
                        ></g>
                        <g>
                            <g>
                                <path d="M80.179,13.758c-18.342-18.342-48.08-18.342-66.422,0c-18.342,18.341-18.342,48.08,0,66.421 c18.342,18.342,48.08,18.342,66.422,0C98.521,61.837,98.521,32.099,80.179,13.758z M44.144,83.117 c-4.057,0-7.001-3.071-7.001-7.305c0-4.291,2.987-7.404,7.102-7.404c4.123,0,7.001,3.044,7.001,7.404 C51.246,80.113,48.326,83.117,44.144,83.117z M54.73,44.921c-4.15,4.905-5.796,9.117-5.503,14.088l0.097,2.495 c0.011,0.062,0.017,0.125,0.017,0.188c0,0.58-0.47,1.051-1.05,1.051c-0.004-0.001-0.008-0.001-0.012,0h-7.867 c-0.549,0-1.005-0.423-1.047-0.97l-0.202-2.623c-0.676-6.082,1.508-12.218,6.494-18.202c4.319-5.087,6.816-8.865,6.816-13.145 c0-4.829-3.036-7.536-8.548-7.624c-3.403,0-7.242,1.171-9.534,2.913c-0.264,0.201-0.607,0.264-0.925,0.173 s-0.575-0.327-0.693-0.636l-2.42-6.354c-0.169-0.442-0.02-0.943,0.364-1.224c3.538-2.573,9.441-4.235,15.041-4.235 c12.36,0,17.894,7.975,17.894,15.877C63.652,33.765,59.785,38.919,54.73,44.921z"></path>
                            </g>
                        </g>
                    </svg>
                    <span class="tooltip min-w-[130px] max-w-[500px] p-1">{info}</span>
                </div>
            </div>
            <Show
                when={move || { selected_option() == Some("Custom".to_string()) }}
                fallback={move || {
                    view! {
                        <select
                            on:input={
                                let callback = on_select.clone();
                                move |e: ev::Event| { callback(e) }
                            }

                            class="w-full bg-dc_nav rounded p-2 h-[36px] outline-none border-[1px] border-[#68696b] hover:border-dc_blue text-dc_white font-normal"
                        >
                            <option
                                prop:selected={selected_option() == Some("None".to_owned())}
                                value="None"
                            >
                                None
                            </option>
                            <option
                                prop:selected={selected_option()
                                    == Some("SinceDiscordStarted".to_owned())}
                                value="SinceDiscordStarted"
                            >
                                Since Discord Started
                            </option>
                            <option
                                prop:selected={selected_option()
                                    == Some("SinceKutsRpcStarted".to_owned())}
                                value="SinceKutsRpcStarted"
                            >
                                Since KutsRPC Started
                            </option>
                            <option
                                prop:selected={selected_option() == Some("LocalTime".to_owned())}
                                value="LocalTime"
                            >
                                Your Local Time
                            </option>
                            <option
                                prop:selected={selected_option()
                                    == Some("SinceLastActivity".to_owned())}
                                value="SinceLastActivity"
                            >
                                Since Last Activity Update
                            </option>
                            <option
                                prop:selected={selected_option() == Some("Custom".to_owned())}
                                value="Custom"
                            >
                                Custom
                            </option>
                            <Show when={move || custom_time().is_some()} fallback={|| ()}>
                                <option
                                    prop:selected={selected_option() == custom_time()}
                                    value={custom_time}
                                >
                                    Custom:
                                    {custom_time}
                                </option>
                            </Show>
                        </select>
                    }
                }}
            >

                <div class="flex">
                    <input
                        class="w-4/5 bg-dc_nav rounded p-2 h-[36px] outline-none border-[1px] border-[#68696b] hover:border-dc_blue text-dc_white font-normal"
                        type="time"
                        step=1
                        value={custom_time}
                        on:input={move |e| { custom_time.set(Some(event_target_value(&e))) }}
                    />

                    <button
                        on:click={move |_| { selected_option.set(custom_time()) }}

                        class="flex items-center justify-center ml-2 rounded-[4px] h-[36px] w-1/5 p-2 transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white"
                    >
                        <span>Done</span>
                    </button>
                </div>
            </Show>
        </div>
    }
}
