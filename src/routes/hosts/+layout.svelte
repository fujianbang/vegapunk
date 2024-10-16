<script lang="ts">
  import AddHostModal from "./AddHostModal.svelte";

  let modalOpen = false;

  function showModal() {
    modalOpen = true;
  }
</script>

<div class="h-screen">
  <div class="p-6">
    <button
      type="button"
      on:click={showModal}
      class="rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
      >Add Host</button
    >
  </div>
  <slot />
</div>

<AddHostModal bind:open={modalOpen} />
