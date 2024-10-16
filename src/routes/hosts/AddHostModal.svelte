<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  export let open: boolean = false;

  let address = "";
  let port = 22;
  let username = "";
  let password = "";

  function connect() {
    // TODO add validation
    // alert(
    //   "Connecting to [" +
    //     address +
    //     ":" +
    //     port +
    //     "] with credentials: " +
    //     username +
    //     ":" +
    //     password
    // );

    invoke("add_new_host", { address, port, username, password })
      .then(() => {
        open = false;
      })
      .catch((e) => alert(e));
  }

  function closeModal() {
    console.log("close");
    open = false;
  }
</script>

{#if open}
  <div
    class="relative z-10"
    aria-labelledby="modal-title"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
      aria-hidden="true"
    ></div>

    <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
      <div
        class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
      >
        <div
          class="relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6"
        >
          <div class="max-w-md space-y-3">
            <div class="relative flex items-center">
              <div class="flex-auto">
                <input
                  bind:value={address}
                  type="text"
                  class="peer py-2 px-3 ps-11 block w-full bg-gray-100 border-transparent rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-700 dark:border-transparent dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600"
                  placeholder="IP or hostname"
                />
                <div
                  class="absolute inset-y-0 start-0 flex items-center pointer-events-none ps-4 peer-disabled:opacity-50 peer-disabled:pointer-events-none"
                >
                  <svg
                    class="shrink-0 size-4 text-gray-500 dark:text-neutral-500"
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 24 24"
                    fill="none"
                    ><path
                      fill="currentColor"
                      d="M4.26 12A8.2 8.2 0 0 1 4 10a8.2 8.2 0 0 1 .26-2h3.38a17 17 0 0 0-.14 2a17 17 0 0 0 .14 2h2.02a15 15 0 0 1-.16-2a15 15 0 0 1 .16-2h4.68a15 15 0 0 1 .16 2a15 15 0 0 1-.16 2h2.02a17 17 0 0 0 .14-2a17 17 0 0 0-.14-2h3.38a8.2 8.2 0 0 1 .26 2a8.2 8.2 0 0 1-.26 2h2.059A10 10 0 1 0 2.2 12Zm14.66-6h-2.95a15.7 15.7 0 0 0-1.38-3.56A8.03 8.03 0 0 1 18.92 6M12 2.04A14.1 14.1 0 0 1 13.91 6h-3.82A14.1 14.1 0 0 1 12 2.04m-2.59.4A15.7 15.7 0 0 0 8.03 6H5.08a8 8 0 0 1 4.33-3.56m3.339 15.055h2v1h-2z"
                    /><path
                      fill="currentColor"
                      d="M20.998 14H3.002A2 2 0 0 0 1 16.002v5.996A2 2 0 0 0 3.002 24h17.996A2 2 0 0 0 23 21.998v-5.996A2 2 0 0 0 20.998 14M9.251 22.005h-1.5v-6h1.5Zm6.998-3.51a1.473 1.473 0 0 1-1.5 1.5h-2v2h-1.5v-6h3.5a1.473 1.473 0 0 1 1.5 1.5Z"
                    /></svg
                  >
                </div>
              </div>

              <div class="relative flex-initial w-32 ml-5">
                <input
                  bind:value={port}
                  type="number"
                  class="peer py-2 px-3 ps-11 block w-full bg-gray-100 border-transparent rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-700 dark:border-transparent dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600"
                  placeholder="Port"
                />
                <div
                  class="absolute inset-y-0 start-0 flex items-center pointer-events-none ps-4 peer-disabled:opacity-50 peer-disabled:pointer-events-none"
                >
                  <svg
                    class="shrink-0 size-4 text-gray-500 dark:text-neutral-500"
                    xmlns="http://www.w3.org/2000/svg"
                    width="1em"
                    height="1em"
                    viewBox="0 0 32 32"
                    ><path
                      fill="currentColor"
                      d="M21 7h-2V3.6c0-.9-.7-1.6-1.6-1.6H4v2h13v3h-2c-3.3 0-6 2.7-6 6v6c0 3.3 2.7 6 6 6h2v3H4v2h13.4c.9 0 1.6-.7 1.6-1.6V25h2c3.3 0 6-2.7 6-6v-6c0-3.3-2.7-6-6-6m4 12c0 2.2-1.8 4-4 4h-6c-2.2 0-4-1.8-4-4v-6c0-2.2 1.8-4 4-4h6c2.2 0 4 1.8 4 4z"
                    /></svg
                  >
                </div>
              </div>
            </div>

            <div class="relative">
              <input
                bind:value={username}
                type="text"
                class="peer py-2 px-3 ps-11 block w-full bg-gray-100 border-transparent rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-700 dark:border-transparent dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600"
                placeholder="Username"
              />
              <div
                class="absolute inset-y-0 start-0 flex items-center pointer-events-none ps-4 peer-disabled:opacity-50 peer-disabled:pointer-events-none"
              >
                <svg
                  class="shrink-0 size-4 text-gray-500 dark:text-neutral-500"
                  xmlns="http://www.w3.org/2000/svg"
                  width="1em"
                  height="1em"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
                  <circle cx="12" cy="7" r="4"></circle>
                </svg>
              </div>
            </div>

            <div class="relative">
              <input
                bind:value={password}
                type="password"
                class="peer py-2 px-3 ps-11 block w-full bg-gray-100 border-transparent rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-700 dark:border-transparent dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600"
                placeholder="Password"
              />
              <div
                class="absolute inset-y-0 start-0 flex items-center pointer-events-none ps-4 peer-disabled:opacity-50 peer-disabled:pointer-events-none"
              >
                <svg
                  class="shrink-0 size-4 text-gray-500 dark:text-neutral-500"
                  xmlns="http://www.w3.org/2000/svg"
                  width="1em"
                  height="1em"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <path
                    d="M2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4a6.5 6.5 0 1 0-4-4Z"
                  ></path>
                  <circle cx="16.5" cy="7.5" r=".5"></circle>
                </svg>
              </div>
            </div>
          </div>
          <div
            class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3"
          >
            <button
              type="button"
              class="inline-flex w-full justify-center rounded-md bg-orange-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-orange-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orange-600 sm:col-start-2"
              on:click={connect}>Connect</button
            >
            <button
              type="button"
              class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:col-start-1 sm:mt-0"
              on:click={closeModal}>Cancel</button
            >
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}
