use crate::{
    components::{
        alert::*,
        config_modal::*,
    },
    invoke,
    utils::load,
    GlobalState,
};
use fancy_regex::Regex;
use leptos::{
    logging::log,
    *,
};
#[component]
pub fn Header() -> impl IntoView {
    let gs = use_context::<GlobalState>().unwrap();
    let app_id = gs.app_id;
    let remember = gs.remember;
    let presets_dir = gs.presets_dir;
    let minimize = gs.minimize;
    let current_preset = gs.current_preset;

    let app_id_value: RwSignal<Option<String>> = create_rw_signal(None);
    let remember_value: RwSignal<bool> = create_rw_signal(false);
    let presets_dir_value: RwSignal<Option<String>> = create_rw_signal(None);
    let minimize_value: RwSignal<bool> = create_rw_signal(false);

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

    let show_config_modal: RwSignal<bool> = create_rw_signal(false);
    let update_config_action = create_action(move |input: &()| {
        let c = crate::Config {
            app_id: {
                if remember_value.get_untracked() {
                    app_id_value.get_untracked()
                } else {
                    None
                }
            },
            remember: remember_value.get_untracked(),
            presets_dir: presets_dir_value.get_untracked(),
            minimize: minimize_value.get_untracked(),
        };

        async move {
            invoke!("update_config", {config: crate::Config = c},
                Result<String, String>
            );
            if presets_dir != presets_dir_value {
                load::config(Some(gs)).await;
                load::presets(gs).await;
            } else {
                load::config(Some(gs)).await;
            }
        }
    });

    create_resource(
        || (),
        move |_| async move {
            match load::config(None).await {
                Ok(c) => {
                    app_id_value.set(c.app_id);
                    remember_value.set(c.remember);
                    presets_dir_value.set(c.presets_dir);
                    minimize_value.set(c.minimize);
                }
                Err(e) => log!("{}", e.to_string()),
            }
        },
    );

    view! {
        <Show when={move || show_config_modal()} fallback={|| ()}>
            <ConfigModal
                on_click={move |e| {
                    if app_id_value().is_some()
                        && !Regex::new(r"^(\d{18}|)$")
                            .unwrap()
                            .is_match(app_id_value().unwrap().as_str())
                            .unwrap()
                    {
                        alerts.update(|a| a[0].0 = true)
                    } else {
                        update_config_action.dispatch(());
                        show_config_modal.set(false);
                        alerts.set(vec![(false, None)]);
                    }
                }}

                on_cancel={move |e| {
                    show_config_modal.set(false);
                    alerts.set(vec![(false, None)]);
                    app_id_value.set(app_id.get_untracked());
                    remember_value.set(remember.get_untracked());
                    presets_dir_value.set(presets_dir.get_untracked());
                    minimize_value.set(minimize.get_untracked());
                }}

                app_id={app_id_value}
                remember={remember_value}
                presets_dir={presets_dir_value}
                minimize={minimize_value}
            />
            <Show when={move || alerts()[0].0} fallback={|| ()}>
                <Alert text={"Please enter a valid App ID or leave the field empty. An App ID should consist of 18 digits."
                    .to_string()}/>
            </Show>
        </Show>
        <div
            id="header"
            class="rounded-b-md relative w-full h-14 bg-dc_nav shadow-dc_black drop-shadow-lg grid grid-cols-3 items-center"
        >

            <div id="config" class="justify-start">
                <button
                    on:click={move |_| show_config_modal.set(true)}
                    class="flex items-center border border-dc_white px-2 py-1 transition-colors rounded-md duration-500 hover:bg-dc_white hover:text-dc_white group text-xs ml-2 hover:drop-shadow-lg hover:shadow-dc_black"
                >
                    <svg
                        class="h-6 w-6 fill-dc_white fill-current group-hover:fill-dc_black flex-shrink-0"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                    >
                        <path
                            xmlns="http://www.w3.org/2000/svg"
                            d="M 10.490234 2 C 10.011234 2 9.6017656 2.3385938 9.5097656 2.8085938 L 9.1757812 4.5234375 C 8.3550224 4.8338012 7.5961042 5.2674041 6.9296875 5.8144531 L 5.2851562 5.2480469 C 4.8321563 5.0920469 4.33375 5.2793594 4.09375 5.6933594 L 2.5859375 8.3066406 C 2.3469375 8.7216406 2.4339219 9.2485 2.7949219 9.5625 L 4.1132812 10.708984 C 4.0447181 11.130337 4 11.559284 4 12 C 4 12.440716 4.0447181 12.869663 4.1132812 13.291016 L 2.7949219 14.4375 C 2.4339219 14.7515 2.3469375 15.278359 2.5859375 15.693359 L 4.09375 18.306641 C 4.33275 18.721641 4.8321562 18.908906 5.2851562 18.753906 L 6.9296875 18.1875 C 7.5958842 18.734206 8.3553934 19.166339 9.1757812 19.476562 L 9.5097656 21.191406 C 9.6017656 21.661406 10.011234 22 10.490234 22 L 13.509766 22 C 13.988766 22 14.398234 21.661406 14.490234 21.191406 L 14.824219 19.476562 C 15.644978 19.166199 16.403896 18.732596 17.070312 18.185547 L 18.714844 18.751953 C 19.167844 18.907953 19.66625 18.721641 19.90625 18.306641 L 21.414062 15.691406 C 21.653063 15.276406 21.566078 14.7515 21.205078 14.4375 L 19.886719 13.291016 C 19.955282 12.869663 20 12.440716 20 12 C 20 11.559284 19.955282 11.130337 19.886719 10.708984 L 21.205078 9.5625 C 21.566078 9.2485 21.653063 8.7216406 21.414062 8.3066406 L 19.90625 5.6933594 C 19.66725 5.2783594 19.167844 5.0910937 18.714844 5.2460938 L 17.070312 5.8125 C 16.404116 5.2657937 15.644607 4.8336609 14.824219 4.5234375 L 14.490234 2.8085938 C 14.398234 2.3385937 13.988766 2 13.509766 2 L 10.490234 2 z M 12 8 C 14.209 8 16 9.791 16 12 C 16 14.209 14.209 16 12 16 C 9.791 16 8 14.209 8 12 C 8 9.791 9.791 8 12 8 z"
                        ></path>
                    </svg>
                    <span class="text-dc_white group-hover:text-dc_black mt-0.5 ml-1">Config</span>
                </button>
            </div>
            <div id="title" class="font-extrabold text-dc_white justify-self-center">
                {move || {
                    if let Some(p) = current_preset() {
                        format!("KutsRPC - {}", p.name).into_view()
                    } else {
                        view! { KutsRPC }.into_view()
                    }
                }}

            </div>
            <div id="social" class="flex justify-end items-center space-x-4 mr-4">
                <a
                    id="discord-link"
                    target="_blank"
                    href="https://discord.com/invite/ANDJzywMAP"
                    class="cursor-pointer"
                >
                    <svg
                        class="transition-colors duration-300 h-6 w-6 fill-dc_white hover:fill-dc_black fill-current"
                        width="64px"
                        height="64px"
                        viewBox="0 -28.5 256 256"
                        version="1.1"
                        xmlns="http://www.w3.org/2000/svg"
                        xmlns:xlink="http://www.w3.org/1999/xlink"
                        preserveAspectRatio="xMidYMid"
                    >
                        <g stroke-width="0"></g>
                        <g stroke-linecap="round" stroke-linejoin="round"></g>
                        <g>
                            <g>
                                <path
                                    d="M216.856339,16.5966031 C200.285002,8.84328665 182.566144,3.2084988 164.041564,0 C161.766523,4.11318106 159.108624,9.64549908 157.276099,14.0464379 C137.583995,11.0849896 118.072967,11.0849896 98.7430163,14.0464379 C96.9108417,9.64549908 94.1925838,4.11318106 91.8971895,0 C73.3526068,3.2084988 55.6133949,8.86399117 39.0420583,16.6376612 C5.61752293,67.146514 -3.4433191,116.400813 1.08711069,164.955721 C23.2560196,181.510915 44.7403634,191.567697 65.8621325,198.148576 C71.0772151,190.971126 75.7283628,183.341335 79.7352139,175.300261 C72.104019,172.400575 64.7949724,168.822202 57.8887866,164.667963 C59.7209612,163.310589 61.5131304,161.891452 63.2445898,160.431257 C105.36741,180.133187 151.134928,180.133187 192.754523,160.431257 C194.506336,161.891452 196.298154,163.310589 198.110326,164.667963 C191.183787,168.842556 183.854737,172.420929 176.223542,175.320965 C180.230393,183.341335 184.861538,190.991831 190.096624,198.16893 C211.238746,191.588051 232.743023,181.531619 254.911949,164.955721 C260.227747,108.668201 245.831087,59.8662432 216.856339,16.5966031 Z M85.4738752,135.09489 C72.8290281,135.09489 62.4592217,123.290155 62.4592217,108.914901 C62.4592217,94.5396472 72.607595,82.7145587 85.4738752,82.7145587 C98.3405064,82.7145587 108.709962,94.5189427 108.488529,108.914901 C108.508531,123.290155 98.3405064,135.09489 85.4738752,135.09489 Z M170.525237,135.09489 C157.88039,135.09489 147.510584,123.290155 147.510584,108.914901 C147.510584,94.5396472 157.658606,82.7145587 170.525237,82.7145587 C183.391518,82.7145587 193.761324,94.5189427 193.539891,108.914901 C193.539891,123.290155 183.391518,135.09489 170.525237,135.09489 Z"
                                    fill-rule="nonzero"
                                ></path>
                            </g>
                        </g>
                    </svg>
                </a>

                <a
                    id="github-link"
                    target="_blank"
                    href="https://github.com/kutsoji/KutsRPC"
                    class="cursor-pointer"
                >
                    <svg
                        class="transition-colors duration-300 h-6 w-6 fill-dc_white hover:fill-dc_black fill-current"
                        width="64px"
                        height="64px"
                        viewBox="0 0 256.00 256.00"
                        xmlns="http://www.w3.org/2000/svg"
                        preserveAspectRatio="xMinYMin meet"
                    >
                        <g stroke-width="0"></g>
                        <g stroke-linecap="round" stroke-linejoin="round"></g>
                        <g>
                            <g>
                                <path d="M127.505 0C57.095 0 0 57.085 0 127.505c0 56.336 36.534 104.13 87.196 120.99 6.372 1.18 8.712-2.766 8.712-6.134 0-3.04-.119-13.085-.173-23.739-35.473 7.713-42.958-15.044-42.958-15.044-5.8-14.738-14.157-18.656-14.157-18.656-11.568-7.914.872-7.752.872-7.752 12.804.9 19.546 13.14 19.546 13.14 11.372 19.493 29.828 13.857 37.104 10.6 1.144-8.242 4.449-13.866 8.095-17.05-28.32-3.225-58.092-14.158-58.092-63.014 0-13.92 4.981-25.295 13.138-34.224-1.324-3.212-5.688-16.18 1.235-33.743 0 0 10.707-3.427 35.073 13.07 10.17-2.826 21.078-4.242 31.914-4.29 10.836.048 21.752 1.464 31.942 4.29 24.337-16.497 35.029-13.07 35.029-13.07 6.94 17.563 2.574 30.531 1.25 33.743 8.175 8.929 13.122 20.303 13.122 34.224 0 48.972-29.828 59.756-58.22 62.912 4.573 3.957 8.648 11.717 8.648 23.612 0 17.06-.148 30.791-.148 34.991 0 3.393 2.295 7.369 8.759 6.117 50.634-16.879 87.122-64.656 87.122-120.973C255.009 57.085 197.922 0 127.505 0"></path>
                                <path d="M47.755 181.634c-.28.633-1.278.823-2.185.389-.925-.416-1.445-1.28-1.145-1.916.275-.652 1.273-.834 2.196-.396.927.415 1.455 1.287 1.134 1.923M54.027 187.23c-.608.564-1.797.302-2.604-.589-.834-.889-.99-2.077-.373-2.65.627-.563 1.78-.3 2.616.59.834.899.996 2.08.36 2.65M58.33 194.39c-.782.543-2.06.034-2.849-1.1-.781-1.133-.781-2.493.017-3.038.792-.545 2.05-.055 2.85 1.07.78 1.153.78 2.513-.019 3.069M65.606 202.683c-.699.77-2.187.564-3.277-.488-1.114-1.028-1.425-2.487-.724-3.258.707-.772 2.204-.555 3.302.488 1.107 1.026 1.445 2.496.7 3.258M75.01 205.483c-.307.998-1.741 1.452-3.185 1.028-1.442-.437-2.386-1.607-2.095-2.616.3-1.005 1.74-1.478 3.195-1.024 1.44.435 2.386 1.596 2.086 2.612M85.714 206.67c.036 1.052-1.189 1.924-2.705 1.943-1.525.033-2.758-.818-2.774-1.852 0-1.062 1.197-1.926 2.721-1.951 1.516-.03 2.758.815 2.758 1.86M96.228 206.267c.182 1.026-.872 2.08-2.377 2.36-1.48.27-2.85-.363-3.039-1.38-.184-1.052.89-2.105 2.367-2.378 1.508-.262 2.857.355 3.049 1.398"></path>
                            </g>
                        </g>
                    </svg>
                </a>
            </div>

        </div>
    }
}
