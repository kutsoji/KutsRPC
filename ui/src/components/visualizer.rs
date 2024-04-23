use crate::{
    components::error_modal::*,
    utils::time::{
        self,
        current_timestamp,
    },
    GlobalState,
};
use leptos::*;

#[component]
pub fn Visualizer() -> impl IntoView {
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
    let actual_timestamp =
        create_resource(timestamp, move |t| async move { time::get_timestamp(t).await });
    let current_timestamp = create_rw_signal(time::current_timestamp());
    set_interval(
        move || current_timestamp.update(|t| *t += 1),
        std::time::Duration::from_millis(1000),
    );
    view! {
        <div class="col-span-5 h-full ">
            <div class="vertical-hr"></div>
            <div class="p-20 relative">
                <div class="bg-dc_blue rounded-t-lg w-[300px] h-[70px] z-10"></div>
                <div class="flex items-end justify-start absolute bottom-[45px] left-[100px] w-[90px] h-[90px]">
                    <img
                        draggable="false"
                        src="../../assets/imgs/test.png"
                        alt="Avatar"
                        class="rounded-full border-solid border-4 border-dc_black z-10 w-full h-full"
                    />
                    <div class="rounded-full bg-dc_green w-[18px] h-[18px] border-solid border-2 border-dc_black absolute bottom-[10px] left-[75px] transform translate-x-[-50%] translate-y-[45%] z-20"></div>
                </div>
                <div class="absolute bottom-0 left-[100px]">
                    <span class="text-dc_white font-bold">kutsoji</span>
                </div>
                <div class="absolute bottom-[-50px] left-[100px]">
                    <span class="text-dc_gray font-semibold text-sm">PLAYING A GAME</span>
                </div>
                <div class="absolute bottom-[-125px] left-[100px] grid grid-cols-8 grid-rows-4 gap-x-2 w-[300px] text-xs font-normal">
                    {move || {
                        if large_img_key().is_some() {
                            view! {
                                <div class="row-span-4 col-span-2 has-tooltip">
                                    <img
                                        prop:src={move || {
                                            if large_img_key() == Some("".to_owned())
                                                || large_img_key() == None
                                            {
                                                "../../assets/imgs/test.png".to_owned()
                                            } else {
                                                large_img_key().unwrap()
                                            }
                                        }}

                                        class="rounded-lg z-10 w-[70px] h-[70px]"
                                    />
                                    <span class="tooltip p-2">{large_img_txt}</span>
                                </div>
                            }
                                .into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                    {move || {
                        if small_img_key().is_some() && large_img_key().is_some() {
                            view! {
                                <div class="has-tooltip absolute bottom-[3px] left-[65px] z-20 transform translate-x-[-50%] translate-y-[45%]">
                                    <img
                                        prop:src={move || {
                                            if small_img_key() == Some("".to_owned())
                                                || small_img_key() == None
                                            {
                                                "../../assets/imgs/test.png".to_owned()
                                            } else {
                                                small_img_key().unwrap()
                                            }
                                        }}

                                        class="bg-dc_nav rounded-full w-[24px] h-[24px] border-solid border-2 border-dc_black"
                                    />
                                    <span class="tooltip p-2">{small_img_txt}</span>
                                </div>
                            }
                                .into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                    <span class="text-dc_white col-span-6 font-semibold">KutsRPC</span>
                    {move || {
                        if details().is_some() {
                            view! {
                                <span class="text-dc_white col-span-6 truncate w-[225px]">
                                    {details}
                                </span>
                            }
                                .into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                    {move || {
                        if state().is_some() {
                            view! {
                                <span class="text-dc_white col-span-6 truncate w-[225px]">
                                    {state}
                                </span>
                            }
                                .into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}
                    {move || {
                        if timestamp().is_some() {
                            view! {
                                <span class="text-dc_white col-span-6 truncate w-[225px]">
                                    {move || {
                                        actual_timestamp
                                            .with(|ti| match ti {
                                                Some(t) => {
                                                    match t {
                                                        Ok(value) => {
                                                            value
                                                                .clone()
                                                                .map(|unix_time| time::get_time(
                                                                    current_timestamp(),
                                                                    unix_time,
                                                                ))
                                                                .into_view()
                                                        }
                                                        Err(e) => {
                                                            view! {
                                                                <ErrorModal
                                                                    title={"Timestamp Selection Error".to_string()}
                                                                    description={e.to_string()}
                                                                    button_title={"OK".to_string()}
                                                                    on_click={move |_| { timestamp.set(None) }}
                                                                />
                                                            }
                                                        }
                                                    }
                                                }
                                                None => ().into_view(),
                                            })
                                    }}

                                </span>
                            }
                                .into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}

                </div>
                <div class="grid grid-cols-2 gap-x-2 absolute left-[100px] bottom-[-175px] w-[250px]">
                    <Show
                        when={move || {
                            first_btn_txt().is_some() && first_btn_url().is_some_and(|s| s != "")
                        }}

                        fallback={|| ()}
                    >
                        <Show
                            when={move || {
                                second_btn_txt().is_some()
                                    && second_btn_url().is_some_and(|s| s != "")
                            }}

                            fallback={move || {
                                view! {
                                    <button class="col-span-2 flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
                                        <span>{first_btn_txt}</span>
                                    </button>
                                }
                            }}
                        >

                            <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
                                <span>{first_btn_txt}</span>
                            </button>
                            <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
                                <span>{second_btn_txt}</span>
                            </button>
                        </Show>
                    </Show>
                </div>

            </div>
        </div>
    }
}
