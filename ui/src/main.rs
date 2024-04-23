#![allow(unused)]
use leptos::*;
mod components;
use components::{
    header::*,
    main::*,
    sidebar::*,
    visualizer::*,
};
mod utils;
#[macro_use]
mod invoke;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
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
}

impl GlobalState {
    fn new(cx: Scope) -> Self {
        Self {
            state: create_rw_signal(cx, Some("Playing Solo (1 of 5)".to_owned())),
            details: create_rw_signal(cx, Some("Competitive".to_owned())),
            timestamp: create_rw_signal(cx, None),
            large_img_key: create_rw_signal(cx, None),
            large_img_txt: create_rw_signal(cx, None),
            small_img_key: create_rw_signal(cx, None),
            small_img_txt: create_rw_signal(cx, None),
            first_btn_txt: create_rw_signal(cx, None),
            first_btn_url: create_rw_signal(cx, None),
            second_btn_txt: create_rw_signal(cx, None),
            second_btn_url: create_rw_signal(cx, None),
        }
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, GlobalState::new(cx));
    view! { cx,
        <div class="flex flex-col h-screen">
            <Header/>
            <div class="grid grid-cols-14 overflow-hidden">
                <Sidebar/>
                <Main/>
                <Visualizer/>
            </div>
        </div>
    }
}
