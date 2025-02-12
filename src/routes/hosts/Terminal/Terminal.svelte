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
            console.log("got data", event);
            terminal.write(event.data);
        };

        // invoke("create_ssh_connection");

        console.log("ready to listen");
        invoke("listen_ssh_data", { sessionId: "2222-test", channel });

        terminal.onData((data) => {
            console.log(data);
            invoke("communicate", { session_id: "communicate-test", data });
        });

        window.addEventListener("resize", () => fitAddon.fit());
    }

    onMount(() => {
        initTerminal();
    });
</script>

<div class="h-full" bind:this={terminalElement}></div>
