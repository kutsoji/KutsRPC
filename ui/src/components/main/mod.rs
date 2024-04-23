use crate::{
    utils::time,
    GlobalState,
};
use leptos::*;
mod input;
use input::*;
mod timestamp_input;
use timestamp_input::*;
use MaybeSignal::{
    Dynamic,
    Static,
};

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    let gs = use_context::<GlobalState>(cx).expect("to have found the GlobalState provided");
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
    view! { cx,
        <div class="col-span-8 h-full relative">
            <div class="vertical-hr"></div>
            <div class="p-10 pt-10 grid grid-cols-2 grid-rows-7 gap-y-1.5 gap-x-4">
                <Input
                    label="STATE"
                    info="set the state"
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
                    <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
                        <span>Save Activity</span>
                    </button>
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
