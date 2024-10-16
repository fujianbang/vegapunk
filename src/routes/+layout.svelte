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
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              class="size-6"
              viewBox="0 0 24 24"
              ><path
                fill="currentColor"
                fill-rule="evenodd"
                d="M9.757 1.983a.75.75 0 0 1-.733.767c-2.822.064-4.15.349-5.028 1.226c-.57.57-.897 1.342-1.07 2.622c-.174 1.301-.176 3.011-.176 5.392c0 2.38.002 4.09.177 5.392c.172 1.28.499 2.051 1.07 2.622c.57.57 1.341.897 2.621 1.07c1.301.174 3.011.176 5.392.176c2.38 0 4.09-.002 5.392-.177c1.28-.172 2.051-.499 2.622-1.07c.877-.876 1.162-2.206 1.226-5.027a.75.75 0 1 1 1.5.034c-.063 2.766-.313 4.702-1.666 6.054c-.895.896-2.036 1.302-3.482 1.496c-1.413.19-3.224.19-5.535.19h-.114c-2.311 0-4.122 0-5.535-.19c-1.446-.194-2.587-.6-3.482-1.496c-.896-.895-1.302-2.036-1.496-3.482c-.19-1.413-.19-3.224-.19-5.535v-.114c0-2.311 0-4.122.19-5.535c.194-1.446.6-2.587 1.496-3.482C4.289 1.563 6.223 1.313 8.99 1.25a.75.75 0 0 1 .767.733m1.887 1.172a6.506 6.506 0 1 1 2.977 10.901a.2.2 0 0 0-.103-.003a.1.1 0 0 0-.044.022L11.38 17.17a1.98 1.98 0 0 1-1.621.569l-1.741-.194a1.57 1.57 0 0 1-.938-.45l-.174-.174a1.57 1.57 0 0 1-.45-.938l-.194-1.74a1.98 1.98 0 0 1 .569-1.622l3.093-3.093a.1.1 0 0 0 .023-.045a.2.2 0 0 0-.003-.103a6.5 6.5 0 0 1 1.7-6.224m8.14 1.061a5.006 5.006 0 0 0-8.387 4.79c.135.527.026 1.143-.412 1.581l-3.093 3.094a.48.48 0 0 0-.139.395l.193 1.74q.004.026.021.043l.174.174a.07.07 0 0 0 .043.02l1.74.194a.48.48 0 0 0 .395-.139l.5-.5l-.92-.911a.75.75 0 1 1 1.054-1.067l.927.917l1.533-1.532c.438-.438 1.054-.547 1.58-.412a5.006 5.006 0 0 0 4.79-8.387m-5.326 1.909a2.417 2.417 0 1 1 3.417 3.417a2.417 2.417 0 0 1-3.417-3.417m2.357 1.06a.917.917 0 1 0-1.297 1.296a.917.917 0 0 0 1.297-1.296"
                clip-rule="evenodd"
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
