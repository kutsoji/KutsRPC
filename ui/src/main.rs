use leptos::*;

mod components;
use components::{
    header::*,
    main::*,
    sidebar::*,
    visualizer::*,
};

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/>}
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex flex-col h-screen">
          <Header/>

          <div class="flex-grow grid grid-cols-14 overflow-hidden">
          <Sidebar/>
          <Main />
          <Visualizer />
          </div>
        </div>
    }
}
