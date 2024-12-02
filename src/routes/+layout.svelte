<script lang="ts">
    import Icon from "@iconify/svelte";
    import "../app.css";

    type MenuItem = {
        id: string,
        icon: string,
        href: string,
        active: boolean
    }

    let menus: MenuItem[] = [
        {
            id: "Monitoring",
            icon: "icon-park-outline:sales-report",
            href: "/",
            active: true
        },
        {
            id: "Hosts",
            icon: "icon-park-outline:server",
            href: "/hosts",
            active: false
        },
        {
            id: "SFTP",
            icon: "icon-park-outline:data-display",
            href: "/sftp",
            active: false
        },
        {
            id: "Keys",
            icon: "icon-park-outline:keyhole",
            href: "/keys",
            active: false
        }
    ]

    function activeMenu(menu: MenuItem) {
        for (let i = 0; i < menus.length; i++) {
            menus[i].active = false;
        }
        menu.active = true;
    }
</script>

<div class="flex h-screen">
    <div class="w-14 overflow-y-auto bg-gray-900">
        <div class="flex h-16 shrink-0 items-center justify-center">
            <Icon class="h-8 w-auto" icon="token-branded:pond"/>
        </div>
        <nav class="mt-8">
            <ul role="list" class="flex flex-col items-center space-y-1">
                {#each menus as menu}
                    <li>
                        <a href="{menu.href}" on:click={() => activeMenu(menu.id) }
                           class="group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6
            {menu.active ? 'bg-gray-800 text-white': 'text-gray-400 hover:text-white hover:bg-gray-800'}"
                        >
                            <Icon class="size-5" icon={menu.icon}/>
                            <span class="sr-only">{menu.id}</span>
                        </a>
                    </li>
                {/each}
            </ul>
        </nav>
    </div>

    <main class="flex-1 h-full">
        <slot></slot>
    </main>
</div>
