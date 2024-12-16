<script lang="ts">
    import AddHostModal from "../routes/hosts/AddHostModal.svelte";
    import {load} from "./load"
    import Icon from "@iconify/svelte";
    import {hostOs} from "$lib/types/host";
    import {invoke_host_list} from "$lib/command";

    let hosts = load().hostInfo;

    invoke_host_list().then(data => {
        console.log(data)
    });


    let osSvgIconMapping = load().osIcon;

    let showAddHostModal = false;

</script>

<div class="flex flex-col divide-y divide-slate-200">
    <div class="p-4 border-gray-200 flex items-center justify-between">
        <span class="font-bold">Instances</span>
        <button
                type="button"
                on:click={() => showAddHostModal = true}
                class="rounded-md bg-white px-2 py-1 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
        >New
        </button>
    </div>
    <div class="divide-y divide-slate-100 overflow-auto">
        {#each hosts as host}
            <div class="p-4 transition hover:bg-sky-100">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <Icon
                                icon={osSvgIconMapping(host.os)}
                        />
                        <p class="text-sm w-40 text-ellipsis overflow-hidden whitespace-nowrap">{host.name}</p>
                    </div>
                    <div>
                        <button type="button"
                                class="rounded bg-indigo-50 px-2 py-1 text-xs font-semibold text-indigo-600 shadow-sm hover:bg-indigo-100">
                            Conn
                        </button>
                    </div>
                </div>

                <div>
                    <div class="mt-2 text-xs text-gray-500">
                        <div class="flex items-center gap-1">
                            <Icon icon="eos-icons:ip"/>
                            <span class="flex">
                                <span>{host.address}</span>
                                {#if host.port !== 22}
                                    <span>:{host.port}</span>
                                {/if}
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        {/each}
    </div>
</div>

<!-- @slot Add Host Modal -->
<AddHostModal bind:open={showAddHostModal}/>
