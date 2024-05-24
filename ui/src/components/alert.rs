use leptos::*;

#[component]
pub fn Alert(text: String) -> impl IntoView {
    view! {
        <div
            class="w-auto fixed inset-x-0 top-6 flex justify-self-center z-50 p-4 text-dc_white rounded-lg bg-dc_black border border-[#a8242a]"
            role="alert"
        >
            <svg
                class="flex-shrink-0 w-5 mt-[2px] mr-2 h-5 fill-[#a8242a]"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                fill="currentColor"
                viewBox="0 0 20 20"
            >
                <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z"></path>
            </svg>
            <div>
                <span class="text-xs font-medium">{text}</span>
            </div>
        </div>
    }
}
