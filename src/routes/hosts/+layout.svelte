<script>
  import List from "./List.svelte";
</script>

<div class="flex h-full">
  <!-- <div class="flex-none w-64 border-r border-gray-200">
    <List />
  </div> -->

  <div class="flex-auto">
    <slot />
  </div>
</div>
