use leptos::*;

#[component]
pub fn Visualizer(cx: Scope) -> impl IntoView {
    view! {cx,
            <div class="col-span-5 h-full">
        <div class="vertical-hr"></div>
        <div class="p-20 relative">
            <div class="bg-dc_blue rounded-t-lg w-[300px] h-[70px] z-10"></div>
            <div class="flex items-end justify-start absolute bottom-[45px] left-[100px] w-[90px] h-[90px]">
                <img draggable=false src="../../assets/imgs/test.png" alt="Avatar" class="rounded-full border-solid border-4 border-dc_black z-10 w-full h-full"/>
                <div class="rounded-full bg-dc_green w-[18px] h-[18px] border-solid border-4 border-dc_black absolute bottom-[10px] left-[75px] transform translate-x-[-50%] translate-y-[45%] z-20"></div>
            </div>
            <div class="absolute bottom-0 left-[100px]">
                <span class="text-dc_white font-bold">kutsoji</span>
            </div>
            <div class="absolute bottom-[-50px] left-[100px]">
                <span class="text-dc_gray font-semibold text-sm">PLAYING A GAME</span>
            </div>
            <div class="absolute bottom-[-125px] left-[100px] grid grid-cols-8 grid-rows-4 gap-x-2 w-[300px] text-xs font-normal">
                <div class="row-span-4 col-span-2 has-tooltip">
                <img src="../../assets/imgs/test.png" alt="Avatar" class="rounded-lg z-10 w-full h-full"/>
                        <span class="tooltip p-2">info</span>
                </div>
                <div class="has-tooltip absolute bottom-[3px] left-[65px] z-20 transform translate-x-[-50%] translate-y-[45%]">
                <div class="rounded-full bg-dc_green w-[20px] h-[20px] border-solid border-2 border-dc_black"></div>
                        <span class="tooltip p-2">info</span>
                </div>
                <span class="text-dc_white col-span-6 font-semibold">KutsRPC</span>
                <span class="text-dc_white col-span-6">Competitive</span>
                <span class="text-dc_white col-span-6">Paying Solo (1 of 15)</span>
                <span class="text-dc_white col-span-6">00:00 left</span>
            </div>
            <div class="grid grid-cols-2 gap-x-2 absolute left-[100px] bottom-[-175px] w-[250px]">
            <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
                <span>discord</span>
            </button>
            <button class="flex justify-center items-center rounded-[4px] h-[30px] p-2 w-full transition-colors duration-500 bg-dc_btn outline-none hover:bg-[#676A75] text-sm font-medium text-dc_white">
                <span>github</span>
            </button>
            </div>

        </div>
    </div>
    }
}
