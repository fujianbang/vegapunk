<script lang="ts">
    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from "@xterm/addon-fit";
    import { onMount, onDestroy } from "svelte";
    import "@xterm/xterm/css/xterm.css";
    import { Channel, invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let terminalElement: HTMLElement;
    let terminal: Terminal;

    interface SshEvent {
        event: "data";
        data: string;
    }

    async function initTerminal() {
        if (!terminalElement) return;

        terminal = new Terminal({
            cursorBlink: true,
            fontSize: 14,
            theme: {
                foreground: "#e0e1e4",
                background: "#181818",
            },
        });

        const fitAddon = new FitAddon();
        terminal.loadAddon(fitAddon);
        terminal.open(terminalElement);
        fitAddon.fit();

        // create channel
        const channel = new Channel<SshEvent>();

        channel.onmessage = (event: SshEvent) => {
            terminal.write(event.data);
        };

        invoke("create_ssh_connection");
        invoke("listen_ssh_data", { ptyChannel: channel });

        // 处理用户输入
        terminal.onData((data) => {
            console.log(data);
            invoke("send_ssh_data", { data });
            terminal.write(data);
        });

        window.addEventListener("resize", () => fitAddon.fit());
    }

    onMount(() => {
        initTerminal();
    });
</script>

<div class="h-full" bind:this={terminalElement}></div>
