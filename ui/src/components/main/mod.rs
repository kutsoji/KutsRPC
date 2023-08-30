use leptos::*;
mod input;
use input::*;

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="col-span-8 h-full relative">
        <div class="vertical-hr"></div>
        <div class="p-10 pt-12 grid grid-cols-2 grid-rows-7 gap-y-2 gap-x-4">
        <Input label="STATE" info="change the state of the activity" input_type="text"/>
        <Input label="DETAILS" info="change the state of the activity" input_type="text"/>
        <Input label="START TIMESTAMP" info="change the state of the activity" input_type="text"/>
        <Input label="END TIMESTAMP" info="change the state of the activity" input_type="text"/>
        <Input label="LARGE IMAGE" info="change the state of the activity" input_type="file"/>
        <Input label="LARGE IMAGE TEXT" info="change the state of the activity" input_type="text"/>
        <Input label="SMALL IMAGE" info="change the state of the activity" input_type="file"/>
        <Input label="SMALL IMAGE TEXT" info="change the state of the activity" input_type="text"/>
        <Input label="BUTTON 1" info="change the state of the activity" input_type="text"/>
        <Input label="URL" info="change the state of the activity" input_type="text"/>
        <Input label="BUTTON 2" info="change the state of the activity" input_type="text"/>
        <Input label="URL" info="change the state of the activity" input_type="text"/>
        <div class="col-span-2 grid grid-cols-3 mt-4 gap-x-2">
        <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
            <span>Set Activity</span>
        </button>
        <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
            <span>Clear Activity</span>
        </button>
        <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
            <span>Save Preset</span>
        </button>
        </div>

        </div>
        </div>
    }
}
