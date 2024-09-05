<script lang="ts">
  export let data;

  import { hostOs } from "$lib/types/host";
  import Icon from "@iconify/svelte";

  let { hostInfo: HostList } = data;

  function osSvgIconMapping(os: hostOs) {
    switch (os) {
      case hostOs.Windows:
        return "devicon:windows8";
      case hostOs.Apple:
        return "devicon:apple";
      case hostOs.Android:
        return "devicon:android";
      case hostOs.Debian:
        return "devicon:debian";
      case hostOs.Ubuntu:
        return "logos:ubuntu";
      case hostOs.Fedora:
        return "devicon:fedora";
      case hostOs.openSUSE:
        return "devicon:opensuse";
      case hostOs.CentOS:
        return "devicon:centos";
      case hostOs.ArchLinux:
        return "devicon:archlinux";
      case hostOs.RedHat:
        return "devicon:kalilinux";
      case hostOs.Kali:
        return "devicon:kalilinux";
      case hostOs.RockyLinux:
        return "devicon:rockylinux";
      default:
        return "devicon:linux";
    }
  }
</script>

<div class="mx-auto max-w-7xl p-8">
  <ul
    role="list"
    class="grid grid-cols-2 gap-x-4 gap-y-6 lg:grid-cols-4 xl:gap-x-6"
  >
    {#each HostList as host}
      <li class="overflow-hidden rounded-xl border border-gray-200">
        <div
          class="flex items-center gap-x-4 border-b border-gray-900/5 bg-gray-50 p-4"
        >
          <Icon
            icon={osSvgIconMapping(host.os)}
            class="h-10 w-10 p-1 flex-none rounded-lg bg-white object-cover ring-1 ring-gray-900/10"
          />

          <div class="text-sm font-medium leading-6 text-gray-900">
            {host.name}
          </div>
          <div class="relative ml-auto">
            <button
              type="button"
              class="-m-2.5 block hover:text-gray-500"
              id="options-menu-0-button"
              aria-expanded="false"
              aria-haspopup="true"
            >
              <Icon icon="ion:terminal" />
            </button>
            <!-- <button
              type="button"
              class="-m-2.5 block hover:text-gray-500" 
              id="options-menu-0-button"
              aria-expanded="false"
              aria-haspopup="true"
            >
              <Icon icon="ion:terminal" />
            </button> -->
          </div>
        </div>
        <dl class="-my-3 divide-y divide-gray-100 px-4 py-3 text-sm leading-3">
          <div class="flex justify-between gap-x-4 py-3">
            <dt class="text-gray-500">IP Address</dt>
            <dd class="text-gray-700">
              <div class="flex">
                <span>
                  {host.address}
                </span>
              </div>
            </dd>
          </div>
        </dl>
      </li>
    {/each}
  </ul>
</div>
