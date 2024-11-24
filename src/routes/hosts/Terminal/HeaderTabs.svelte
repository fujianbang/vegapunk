<script lang="ts">
  interface activeNodeProps {
    name: string;
    current: boolean;
  }

  let nodeProps: activeNodeProps[] = [
    { name: "Server 01", current: true },
    { name: "192.168.0.1", current: false },
    { name: "Billing", current: false },
  ];
</script>

<div>
  <nav
    class="isolate flex divide-x divide-gray-200 rounded-lg shadow"
    aria-label="Tabs"
  >
    {#each nodeProps as { name, current }}
      <a
        href="#"
        class="group relative min-w-0 overflow-hidden bg-white px-4 py-2 text-center text-sm font-medium hover:bg-gray-50 focus:z-10"
      >
        <span>{name}</span>
        <span
          aria-hidden="true"
          class="absolute inset-x-0 top-0 h-0.5
           {current
            ? 'bg-indigo-500 text-gray-900'
            : 'bg-transparent text-gray-500 hover:text-gray-700'}"
        ></span>
      </a>
    {/each}
  </nav>
</div>
