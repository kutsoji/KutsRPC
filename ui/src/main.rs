#![allow(unused)]
use leptos::*;
mod structs;
use structs::*;
mod components;
use components::{
    header::*,
    main::*,
    sidebar::*,
    visualizer::*,
};

use crate::utils::load;
mod utils;
#[macro_use]
mod invoke;

fn main() {
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalState {
    state: RwSignal<Option<String>>,
    details: RwSignal<Option<String>>,
    timestamp: RwSignal<Option<String>>,
    large_img_key: RwSignal<Option<String>>,
    large_img_txt: RwSignal<Option<String>>,
    small_img_key: RwSignal<Option<String>>,
    small_img_txt: RwSignal<Option<String>>,
    first_btn_txt: RwSignal<Option<String>>,
    first_btn_url: RwSignal<Option<String>>,
    second_btn_txt: RwSignal<Option<String>>,
    second_btn_url: RwSignal<Option<String>>,
    app_id: RwSignal<Option<String>>,
    remember: RwSignal<bool>,
    presets_dir: RwSignal<Option<String>>,
    minimize: RwSignal<bool>,
    presets: RwSignal<Vec<Preset>>,
    current_preset: RwSignal<Option<Preset>>,
}

impl GlobalState {
    fn new() -> Self {
        Self {
            state: create_rw_signal(Some("Playing Solo (1 of 5)".to_owned())),
            details: create_rw_signal(Some("Competitive".to_owned())),
            timestamp: create_rw_signal(None),
            large_img_key: create_rw_signal(None),
            large_img_txt: create_rw_signal(None),
            small_img_key: create_rw_signal(None),
            small_img_txt: create_rw_signal(None),
            first_btn_txt: create_rw_signal(None),
            first_btn_url: create_rw_signal(None),
            second_btn_txt: create_rw_signal(None),
            second_btn_url: create_rw_signal(None),
            app_id: create_rw_signal(None),
            remember: create_rw_signal(false),
            presets_dir: create_rw_signal(None),
            minimize: create_rw_signal(false),
            presets: create_rw_signal(Vec::new()),
            current_preset: create_rw_signal(None),
        }
    }

    pub fn default_activity(&self) -> () {
        self.state.set(Some("Playing Solo (1 of 5)".to_owned()));
        self.details.set(Some("Competitive".to_owned()));
        self.timestamp.set(None);
        self.large_img_key.set(None);
        self.large_img_txt.set(None);
        self.small_img_key.set(None);
        self.small_img_txt.set(None);
        self.first_btn_txt.set(None);
        self.first_btn_url.set(None);
        self.second_btn_txt.set(None);
        self.second_btn_url.set(None);
    }
}

#[component]
fn App() -> impl IntoView {
    provide_context(GlobalState::new());
    let gs = use_context::<GlobalState>().unwrap();
    create_resource(
        || (),
        move |_| async move {
            load::config(Some(gs)).await;
            load::presets(gs).await;
        },
    );

    view! {
        <div on:contextmenu={move |e| e.prevent_default()} class="flex flex-col h-screen">
            <Header/>
            <div class="grid grid-cols-14 overflow-hidden">
                <Sidebar/>
                <Main/>
                <Visualizer/>
            </div>
        </div>
    }
}
