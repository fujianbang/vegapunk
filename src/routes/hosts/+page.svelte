<script lang="ts">
    import {createTabs, melt} from '@melt-ui/svelte';
    import {cubicInOut} from 'svelte/easing';
    import {crossfade} from 'svelte/transition';
    import Terminal from "./Terminal/Terminal.svelte";
    import InstanceSidebar from "./InstanceSidebar/InstanceSidebar.svelte";

    const {
        elements: {root, list, content, trigger},
        states: {value},
    } = createTabs({
        defaultValue: 'tab-1',
    });

    const triggers = [
        {id: 'tab-1', title: 'Local QA'},
        {id: 'tab-2', title: '127.0.0.1'},
        {id: 'tab-3', title: '10.0.0.1'},
    ];

    const [send, receive] = crossfade({
        duration: 250,
        easing: cubicInOut,
    });
</script>
<div class="flex h-full">
    <div class="flex-none w-[266px] h-full overflow-hidden">
        <InstanceSidebar/>
    </div>

    <div class="flex-1 bg-cyan-50 h-full">
        <div use:melt={$root}>
            <div use:melt={$list} class="flex items-center bg-[#292929] text-[#C2C2C2] ">
                {#each triggers as triggerItem}
                    <button use:melt={$trigger(triggerItem.id)}
                            class="p-2 text-sm {triggerItem.id === $value ? 'bg-[#181818]' : ''}">
                        {triggerItem.title}
                        {#if $value === triggerItem.id}
                            <div in:send={{ key: 'trigger' }} out:receive={{ key: 'trigger' }}
                                 class="absolute bottom-1 left-1/2 h-1 w-6 -translate-x-1/2 rounded-full bg-white"
                            />
                        {/if}
                    </button>
                {/each}
            </div>
            <div use:melt={$content('tab-1')} class="grow bg-white">
                <Terminal/>
            </div>
            <div use:melt={$content('tab-2')} class="grow bg-white">
                <Terminal/>
            </div>
            <div use:melt={$content('tab-3')} class="grow bg-white">
                <Terminal/>
            </div>
        </div>

    </div>
</div>

