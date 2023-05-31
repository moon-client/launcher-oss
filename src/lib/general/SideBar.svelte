<script lang="ts">
    import {goto} from "$app/navigation";
    import {ArrowLeftOnRectangle, ChevronUpDown, Cog6Tooth, GlobeAlt, InformationCircle} from "@steeze-ui/heroicons";
    import {Popover, PopoverButton, PopoverPanel,} from "@rgossiaux/svelte-headlessui";
    import {fly} from "svelte/transition";

    import {createPopperActions} from "svelte-popperjs";
    import {Icon} from "@steeze-ui/svelte-icon";
    import SidebarRedirectionButton from "$lib/general/SidebarRedirectionButton.svelte";
    import SidebarButton from "$lib/general/SidebarButton.svelte";
    import {get} from "svelte/store";
    import {UserContext, userContext} from "../../stores";

    let context: UserContext = get(userContext);
    // Make sure to subscribe to future changes (if any happen, very unlikely)
    userContext.subscribe(value => {
        context = value;
    });

    const [popperRef, popperContent] = createPopperActions();

    // Example Popper configuration
    const popperOptions = {
        position: "bottom-end",
        strategy: "absolute",
    };

    function log_out() {
        goto("/");
    }
</script>

<div class="fixed top-0 flex flex-col items-center bg-slate-700/[0.25] border-r border-slate-50/[0.15] min-w-fit w-60 h-full px-2 pt-3 pb-0 shadow-xl overflow-hidden"
     style="backdrop-filter: blur(100px);">
    <div class="flex flex-row self-start items-center justify-center mb-6 mt-1.5 ml-4">
        <img width="25px" src="https://moonclient.xyz/logo.png" alt="branding"/>
        <p class="ml-3 text-lg text-gray-200"><b style='font-weight: 700'>Moon</b> Client</p>
    </div>

    <div class="flex flex-col gap-y-2 w-full">
        <SidebarRedirectionButton icon={ GlobeAlt } text="Launch" url="/launcher"/>
    </div>

    <div class="w-full gap-2 items-center mt-auto">
        <SidebarRedirectionButton class="mb-1" low="true" icon={ Cog6Tooth } text="Settings" url="/launcher/settings"/>
        <SidebarRedirectionButton class="mb-3" low="true" icon={ InformationCircle } text="About"
                                  url="/launcher/about"/>
        <Popover class="relative" style="max-width: 15rem;" let:open>
            <PopoverButton class="inline-flex w-full" use={[popperRef]}>
                <div class="grow border-t border-slate-600/[0.9] flex items-center space-x-2 py-3 pr-2">
                    <div class="inline-flex items-center justify-center w-9 h-9 overflow-hidden rounded-full bg-slate-600">
                        <span class="font-medium text-gray-600 dark:text-gray-300">{context.username.substring(0, 1).toUpperCase()}</span>
                    </div>
                    <div class="flex-grow text-left dark:text-white">
                        <div class="text-white font-semibold overflow-ellipsis overflow-hidden whitespace-nowrap text-sm"
                             style="max-width: 8rem">
                            {context.username}
                        </div>
                        <div class="text-gray-400 font-semibold overflow-ellipsis overflow-hidden whitespace-nowrap"
                             style="font-size: 0.6rem">
                            {context.rank}
                        </div>
                    </div>
                    <Icon class="shrink-0 w-5 h-5" src={ ChevronUpDown }/>
                </div>
            </PopoverButton>
            {#if open}
                <div transition:fly="{{ y: 5, duration: 200 }}">
                    <PopoverPanel static use={[[popperContent, popperOptions]]}
                                  class="bg-slate-700 border border-slate-50/[0.15] rounded-lg mb-96 w-full z-10 p-2">
                        <SidebarButton class="w-full" low="true" icon={ ArrowLeftOnRectangle } text="Logout"/>
                    </PopoverPanel>
                </div>
            {/if}
        </Popover>
    </div>
</div>