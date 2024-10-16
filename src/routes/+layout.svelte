<script lang="ts">
  import Icon from "@iconify/svelte";
  import "../app.css";

  let teams = [
    {
      name: "My Company",
    },
    {
      name: "Labs",
    },
  ];

  let activeMenuItem = "monitoring";

  function handleClickMenu(menuItem: string) {
    activeMenuItem = menuItem;
  }
</script>

<div>
  <!-- Static sidebar for desktop -->
  <div
    class=" fixed inset-y-0 left-0 z-50 block w-20 overflow-y-auto bg-gray-900 pb-4"
  >
    <div class="flex h-16 shrink-0 items-center justify-center">
      <Icon class="h-8 w-auto" icon="token-branded:pond" />
    </div>
    <nav class="mt-8">
      <ul role="list" class="flex flex-col items-center space-y-1">
        <li>
          <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->
          <a
            href="/"
            on:click={() => handleClickMenu("monitoring")}
            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
            {activeMenuItem == 'monitoring'
              ? 'bg-gray-800 text-white'
              : 'text-gray-400 hover:text-white hover:bg-gray-800'}"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="size-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M10.5 6a7.5 7.5 0 1 0 7.5 7.5h-7.5V6Z"
              />
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M13.5 10.5H21A7.5 7.5 0 0 0 13.5 3v7.5Z"
              />
            </svg>
            <span class="sr-only">Monitoring</span>
          </a>
        </li>
        <li>
          <a
            href="/hosts"
            on:click={() => handleClickMenu("hosts")}
            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
           {activeMenuItem == 'hosts'
              ? 'bg-gray-800 text-white'
              : 'text-gray-400 hover:text-white hover:bg-gray-800'}"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="size-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M5.25 14.25h13.5m-13.5 0a3 3 0 0 1-3-3m3 3a3 3 0 1 0 0 6h13.5a3 3 0 1 0 0-6m-16.5-3a3 3 0 0 1 3-3h13.5a3 3 0 0 1 3 3m-19.5 0a4.5 4.5 0 0 1 .9-2.7L5.737 5.1a3.375 3.375 0 0 1 2.7-1.35h7.126c1.062 0 2.062.5 2.7 1.35l2.587 3.45a4.5 4.5 0 0 1 .9 2.7m0 0a3 3 0 0 1-3 3m0 3h.008v.008h-.008v-.008Zm0-6h.008v.008h-.008v-.008Zm-3 6h.008v.008h-.008v-.008Zm0-6h.008v.008h-.008v-.008Z"
              />
            </svg>
            <span class="sr-only">Hosts</span>
          </a>
        </li>
        <li>
          <a
            href="/sftp"
            on:click={() => handleClickMenu("sftp")}
            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
          {activeMenuItem == 'sftp'
              ? 'bg-gray-800 text-white'
              : 'text-gray-400 hover:text-white hover:bg-gray-800'}"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="size-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z"
              />
            </svg>
            <span class="sr-only">SFTP</span>
          </a>
        </li>
        <li>
          <a
            href="/keys"
            on:click={() => handleClickMenu("keys")}
            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
          {activeMenuItem == 'keys'
              ? 'bg-gray-800 text-white'
              : 'text-gray-400 hover:text-white hover:bg-gray-800'}"
          >
            <svg
              class="h-6 w-6 shrink-0"
              viewBox="0 0 15 15"
              stroke-width="1.5"
              aria-hidden="true"
              ><path
                fill="currentColor"
                d="M2 12.5v.5h1v-.5zm5 0v.5h1v-.5zm-4 0V12H2v.5zm4-.5v.5h1V12zm-2-2a2 2 0 0 1 2 2h1a3 3 0 0 0-3-3zm-2 2a2 2 0 0 1 2-2V9a3 3 0 0 0-3 3zm2-8a2 2 0 0 0-2 2h1a1 1 0 0 1 1-1zm2 2a2 2 0 0 0-2-2v1a1 1 0 0 1 1 1zM5 8a2 2 0 0 0 2-2H6a1 1 0 0 1-1 1zm0-1a1 1 0 0 1-1-1H3a2 2 0 0 0 2 2zM1.5 3h12V2h-12zm12.5.5v8h1v-8zm-.5 8.5h-12v1h12zM1 11.5v-8H0v8zm.5.5a.5.5 0 0 1-.5-.5H0A1.5 1.5 0 0 0 1.5 13zm12.5-.5a.5.5 0 0 1-.5.5v1a1.5 1.5 0 0 0 1.5-1.5zM13.5 3a.5.5 0 0 1 .5.5h1A1.5 1.5 0 0 0 13.5 2zm-12-1A1.5 1.5 0 0 0 0 3.5h1a.5.5 0 0 1 .5-.5zM9 6h3V5H9zm0 3h3V8H9z"
              /></svg
            >
            <span class="sr-only">Keys</span>
          </a>
        </li>
        <li>
          <a
            href="/port-forwarding"
            on:click={() => handleClickMenu("port-forwarding")}
            class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
          {activeMenuItem == 'port-forwarding'
              ? 'bg-gray-800 text-white'
              : 'text-gray-400 hover:text-white hover:bg-gray-800'}"
          >
            <svg
              class="h-6 w-6 shrink-0"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 00-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 01-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5a3.375 3.375 0 00-3.375-3.375H9.75"
              />
            </svg>
            <span class="sr-only">Port Forwarding</span>
          </a>
        </li>
      </ul>
    </nav>
  </div>

  <main class="pl-20 h-screen">
    <slot></slot>
  </main>
</div>
