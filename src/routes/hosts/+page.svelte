<script lang="ts">
    import InstanceSidebar from "./InstanceSidebar/InstanceSidebar.svelte";
    import Terminal from "./Terminal/Terminal.svelte";

</script>


<div class="flex h-full">
    <div class="flex-none w-[266px] h-full overflow-hidden">
        <InstanceSidebar/>
    </div>

    <div class="flex-1 bg-cyan-50 h-full">
        <Terminal/>
    </div>
</div>
