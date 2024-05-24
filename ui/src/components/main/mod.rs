use std::fmt::format;

use crate::{
    components::{
        alert::*,
        create_preset_modal::*,
        error_modal::*,
    },
    invoke,
    utils::{
        load,
        time,
    },
    Activity,
    GlobalState,
    Preset,
};
use leptos::{
    logging::log,
    *,
};
mod input;
pub use input::*;
mod timestamp_input;
use fancy_regex::Regex;
use serde::Serialize;
use timestamp_input::*;
use MaybeSignal::{
    Dynamic,
    Static,
};

#[component]
pub fn Main() -> impl IntoView {
    let gs = use_context::<GlobalState>().expect("to have found the GlobalState provided");
    let state = gs.state;
    let details = gs.details;
    let timestamp = gs.timestamp;
    let large_img_key = gs.large_img_key;
    let large_img_txt = gs.large_img_txt;
    let small_img_key = gs.small_img_key;
    let small_img_txt = gs.small_img_txt;
    let first_btn_txt = gs.first_btn_txt;
    let first_btn_url = gs.first_btn_url;
    let second_btn_txt = gs.second_btn_txt;
    let second_btn_url = gs.second_btn_url;
    let presets_dir = gs.presets_dir;
    let presets = gs.presets;
    let current_preset = gs.current_preset;

    let alerts: RwSignal<Vec<(bool, Option<()>)>> = create_rw_signal(vec![(false, None); 2]);
    create_effect(move |_| {
        for (i, (alrt, tm)) in alerts.get().iter().enumerate() {
            if *alrt && tm.is_none() {
                alerts.update(|alrts| {
                    alrts[i].1 = Some(set_timeout(
                        move || {
                            alerts.update(|alrts| alrts[i].0 = false);
                        },
                        std::time::Duration::from_millis(5000),
                    ))
                });
            } else if *alrt && tm.is_some() {
                tm.unwrap()
            } else if !*alrt && tm.is_some() {
                alerts.update(|alrts| alrts[i].1 = None);
            } else {
                ()
            }
        }
    });

    create_effect(move |_| match current_preset() {
        Some(p) => {
            state.set(p.activity.state);
            details.set(p.activity.details);
            timestamp.set(p.activity.timestamp);
            large_img_key.set(p.activity.large_img_key);
            large_img_txt.set(p.activity.large_img_txt);
            small_img_key.set(p.activity.small_img_key);
            small_img_txt.set(p.activity.small_img_txt);
            first_btn_txt.set(p.activity.first_btn_txt);
            first_btn_url.set(p.activity.first_btn_url);
            second_btn_txt.set(p.activity.second_btn_txt);
            second_btn_url.set(p.activity.second_btn_url);
        }
        None => gs.default_activity(),
    });

    let selected_preset_picture: RwSignal<Option<String>> = create_rw_signal(None);
    let preset_name: RwSignal<Option<String>> = create_rw_signal(None);

    let create_preset_action = create_action(move |input: &()| {
        let p = Preset {
            id: time::current_timestamp(),
            name: preset_name.get_untracked().unwrap_or("preset".to_string()),
            picture: selected_preset_picture.get_untracked(),
            activity: Activity {
                state: state.get_untracked(),
                details: details.get_untracked(),
                timestamp: timestamp.get_untracked(),
                large_img_key: large_img_key.get_untracked(),
                large_img_txt: large_img_txt.get_untracked(),
                small_img_key: small_img_key.get_untracked(),
                small_img_txt: small_img_txt.get_untracked(),
                first_btn_txt: first_btn_txt.get_untracked(),
                first_btn_url: first_btn_url.get_untracked(),
                second_btn_txt: second_btn_txt.get_untracked(),
                second_btn_url: second_btn_url.get_untracked(),
            },
            path: format!(
                "{}\\{}.toml",
                presets_dir.get_untracked().unwrap(),
                preset_name.get_untracked().unwrap_or("preset".to_string())
            ),
        };

        async move {
            invoke!("create_preset", {preset: Preset = p},
                Result<String, String>
            );
            load::presets(gs).await;
        }
    });

    let show_create_preset_modal: RwSignal<bool> = create_rw_signal(false);
    view! {
        <div class="col-span-8 h-full relative">
            <div class="vertical-hr"></div>
            <div class="p-10 pt-10 grid grid-cols-2 grid-rows-7 gap-y-1.5 gap-x-4">
                <Input
                    label="STATE"
                    info="set the state"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            state.set(None)
                        } else {
                            state.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(state))}
                />

                <Input
                    label="DETAILS"
                    info="set the details"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            details.set(None)
                        } else {
                            details.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(details))}
                />
                <TimestampInput
                    label="TIMESTAMP"
                    info="set the time left/elapsed, if it is future it shows 'left' otherwise it shows 'elapsed'"
                    on_select={move |e| {
                        if event_target_value(&e) == "None" {
                            timestamp.set(None)
                        } else {
                            timestamp.set(Some(event_target_value(&e)))
                        }
                    }}

                    selected_option={timestamp}
                />
                <Input
                    label="LARGE IMAGE"
                    info="set the large image key (can be a url as well)"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            large_img_key.set(None)
                        } else {
                            large_img_key.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(large_img_key))}
                />
                <Input
                    label="LARGE IMAGE TEXT"
                    info="set the tooltip text for the large image"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            large_img_txt.set(None)
                        } else {
                            large_img_txt.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(large_img_txt))}
                />
                <Input
                    label="SMALL IMAGE"
                    info="set the small image key (can be a url as well)"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            small_img_key.set(None)
                        } else {
                            small_img_key.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(small_img_key))}
                />
                <Input
                    label="SMALL IMAGE TEXT"
                    info="set the tooltip text for the small image"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            small_img_txt.set(None)
                        } else {
                            small_img_txt.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(small_img_txt))}
                />
                <Input
                    label="BUTTON 1"
                    info="set the first button text"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            first_btn_txt.set(None)
                        } else {
                            first_btn_txt.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(first_btn_txt))}
                />

                <Input
                    label="URL"
                    info="set the first button's url"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            first_btn_url.set(None)
                        } else {
                            first_btn_url.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(first_btn_url))}
                />
                <Input
                    label="BUTTON 2"
                    info="set the second button text"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            second_btn_txt.set(None)
                        } else {
                            second_btn_txt.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(second_btn_txt))}
                />

                <Input
                    label="URL"
                    info="set the second button's url"
                    main_class="col-span-1"
                    input_class="input-text"
                    on_input={Box::new(move |e| {
                        if event_target_value(&e) == "" {
                            second_btn_url.set(None)
                        } else {
                            second_btn_url.set(Some(event_target_value(&e)))
                        }
                    })}

                    input_value={Dynamic(Signal::from(second_btn_url))}
                />

                <div class="col-span-2 grid grid-cols-3 mt-4 gap-x-2">
                    <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
                        <span>Set Activity</span>
                    </button>
                    <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
                        <span>Clear Activity</span>
                    </button>
                    <button
                        on:click={move |_| {
                            show_create_preset_modal.set(true);
                        }}

                        class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white"
                    >
                        <span>Save Activity</span>
                    </button>
                    <Show when={move || show_create_preset_modal()} fallback={|| ()}>
                        <CreatePresetModal
                            on_click={move |e| {
                                if preset_name().is_none()
                                    || !Regex::new(r#"^(?!\s+$)(?!.*[\/\\:?*<>|"']).+$"#)
                                        .unwrap()
                                        .is_match(preset_name.get_untracked().unwrap().as_str())
                                        .unwrap()
                                {
                                    alerts.update(|a| a[0].0 = true)
                                } else if presets()
                                    .iter()
                                    .find(|p| p.name == preset_name.get_untracked().unwrap())
                                    .is_some()
                                {
                                    alerts.update(|a| a[1].0 = true)
                                } else {
                                    create_preset_action.dispatch(());
                                    show_create_preset_modal.set(false);
                                    selected_preset_picture.set(None);
                                    preset_name.set(None);
                                    alerts.set(vec![(false, None); 2]);
                                }
                            }}

                            on_cancel={move |e| {
                                show_create_preset_modal.set(false);
                                preset_name.set(None);
                                selected_preset_picture.set(None);
                                alerts.set(vec![(false, None); 2]);
                            }}

                            name={preset_name}
                            selected_picture={selected_preset_picture}
                            path={presets_dir}
                        />

                    </Show>
                    <Show when={move || alerts()[0].0} fallback={|| ()}>
                        <Alert text={r#"Preset name field can neither be empty nor include any of the following characters: /\\*"<>:?"#
                            .to_string()}/>
                    </Show>
                    <Show when={move || alerts()[1].0} fallback={|| ()}>
                        <Alert text={r#"A preset with this name already exists."#.to_string()}/>
                    </Show>
                </div>
                <div class="col-span-2 grid grid-cols-2 mt-4 gap-x-2">
                    <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-200 bg-dc_green hover:bg-[#5baf77] outline-none text-sm font-medium text-dc_white">
                        <span>Connect</span>
                    </button>
                    <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-200 bg-[#a8242a] hover:bg-[#c53d3d] outline-none text-sm font-medium text-dc_white">
                        <span>Disconnect</span>
                    </button>
                </div>

            </div>
        </div>
    }
}
