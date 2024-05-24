use leptos::{
    logging::log,
    *,
};

use crate::{
    components::sidebaricon::*,
    invoke,
    GlobalState,
    Preset,
};

#[component]
pub fn Sidebar() -> impl IntoView {
    let gs = use_context::<GlobalState>().unwrap();
    let presets = gs.presets;
    let presets_dir = gs.presets_dir;
    let current_preset = gs.current_preset;

    let add_preset_action = create_action(move |input: &()| async move {
        match invoke!("add_preset", {presetsDir: String = presets_dir.get_untracked().unwrap()}, Preset)
        {
            Ok(preset) => {
                if !presets().iter().any(|ps| ps.name == preset.name) {
                    presets.update(|mutable_presets: &mut Vec<Preset>| {
                        mutable_presets.push(preset.clone())
                    })
                }
                current_preset.set(Some(preset.clone()));
            }
            Err(e) => log!("{}", e.to_string()),
        }
    });

    let remove_preset_action = create_action(|preset_path: &String| {
        let preset_path = preset_path.to_owned();
        async move {
            match invoke!("remove_preset", {presetPath: String = preset_path.to_string()}, String) {
                Ok(_) => (),
                Err(e) => log!("{}", e.to_string()),
            }
        }
    });
    view! {
        <div class="col-span-1 h-screen">
            <div
                on:click={move |_| {
                    current_preset.set(None);
                }}

                class={move || {
                    "sidebaricons bg-dc_nav hover:bg-dc_blue group ".to_owned()
                        + if current_preset().is_none() { "active-blue" } else { "" }
                }}
            >

                <svg
                    class="duration-150 ease-linear fill-dc_white"
                    width="30px"
                    height="30px"
                    viewBox="0 0 30 30"
                    role="img"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path d="M25.396 5.462a24.739 24.739 0 0 0 -6.106 -1.893 0.092 0.092 0 0 0 -0.099 0.046c-0.264 0.469 -0.555 1.08 -0.761 1.562a22.839 22.839 0 0 0 -6.858 0 15.799 15.799 0 0 0 -0.77 -1.562 0.095 0.095 0 0 0 -0.099 -0.046A24.671 24.671 0 0 0 4.596 5.462a0.088 0.088 0 0 0 -0.041 0.034C0.666 11.308 -0.401 16.976 0.124 22.571a0.103 0.103 0 0 0 0.039 0.072 24.874 24.874 0 0 0 7.49 3.786 0.099 0.099 0 0 0 0.105 -0.034 17.612 17.612 0 0 0 1.532 -2.492 0.094 0.094 0 0 0 -0.05 -0.133 16.383 16.383 0 0 1 -2.34 -1.114 0.095 0.095 0 0 1 -0.011 -0.161 12.75 12.75 0 0 0 0.465 -0.364 0.092 0.092 0 0 1 0.095 -0.013c4.909 2.24 10.226 2.24 15.077 0a0.092 0.092 0 0 1 0.099 0.013c0.15 0.122 0.309 0.246 0.466 0.364a0.095 0.095 0 0 1 -0.006 0.16 15.374 15.374 0 0 1 -2.341 1.114 0.095 0.095 0 0 0 -0.05 0.134c0.45 0.872 0.964 1.704 1.531 2.491a0.094 0.094 0 0 0 0.105 0.034 24.798 24.798 0 0 0 7.502 -3.786 0.095 0.095 0 0 0 0.041 -0.069c0.626 -6.47 -1.048 -12.092 -4.437 -17.074a0.076 0.076 0 0 0 -0.039 -0.039zM10.024 19.164c-1.48 0 -2.697 -1.355 -2.697 -3.024 0 -1.666 1.196 -3.024 2.697 -3.024 1.513 0 2.719 1.369 2.697 3.026 0 1.666 -1.196 3.021 -2.697 3.021zm9.969 0c-1.48 0 -2.697 -1.355 -2.697 -3.024 0 -1.666 1.194 -3.024 2.697 -3.024 1.513 0 2.719 1.369 2.697 3.026 0 1.666 -1.183 3.021 -2.697 3.021z"></path>
                </svg>
            </div>
            <hr class="sidebar-hr"/>
            <div class="h-[80%] overflow-y-auto overflow-x-visible inset-0 top-[20%] absolute">
                <For
                    each={move || presets().into_iter().enumerate()}
                    key={|(idx, p)| (*idx, p.name.clone())}
                    children={move |(i, preset)| {
                        let stored_preset = store_value(preset.clone());
                        view! {
                            <SideBarIcon
                                preset={preset}
                                i={i}
                                current_preset={current_preset}
                                on_click={move |_| { current_preset.set(Some(stored_preset())) }}

                                on_remove={move |_| {
                                    remove_preset_action.dispatch(stored_preset().path);
                                    if let Some(p) = current_preset() {
                                        if p.name == stored_preset().name {
                                            current_preset.set(None)
                                        }
                                    }
                                    presets
                                        .update(|ps| {
                                            ps.remove(i);
                                        });
                                }}
                            />
                        }
                    }}
                />

                <div
                    on:click={move |_| add_preset_action.dispatch(())}
                    class="sidebaricons bg-dc_nav group has-tooltip"
                >
                    <svg
                        class="group-hover:fill-dc_white duration-150 ease-linear fill-dc_green"
                        width="30px"
                        height="30px"
                        viewBox="0 0 30 30"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path d="M24 15c0 0.83 -0.072 1.5 -0.901 1.5H16.5v6.598c0 0.828 -0.67 0.901 -1.5 0.901 -0.83 0 -1.5 -0.074 -1.5 -0.901V16.5H6.902C6.074 16.5 6 15.83 6 15c0 -0.83 0.074 -1.5 0.901 -1.5H13.5V6.902C13.5 6.072 14.17 6 15 6c0.83 0 1.5 0.072 1.5 0.901V13.5h6.598c0.83 0 0.901 0.67 0.901 1.5z"></path>
                    </svg>
                    <span class="tooltip-preset">Add a saved preset</span>
                </div>
            </div>
        </div>
    }
}
