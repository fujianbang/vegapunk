<script lang="ts">
    import {Terminal} from "@xterm/xterm";
    import {FitAddon} from "@xterm/addon-fit";
    import {onMount} from "svelte";
    import "@xterm/xterm/css/xterm.css";
    import {AttachAddon} from '@xterm/addon-attach';


    let terminalObj: HTMLElement;

    function packageMessage(message: string) {
        return JSON.stringify({
            type: "data",
            payload: message,
        });
    }

    function initTerminal() {
        if (!terminalObj) {
            alert("No terminal found");
            return;
        }

        const term = new Terminal({
            cursorBlink: true,
            fontSize: 14,
            theme: {
                foreground: "#e0e1e4", //字体
                background: "#181818",
            },
        });
        // addon
        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        fitAddon.fit();

        const ws = new WebSocket("ws://localhost:8080/socket?instance_id=a706fe14-d770-4e74-813f-1bc592476eda&container=nginx");
        ws.onopen = function () {
            console.log("open");
        }

        ws.onclose = function () {
            console.log("close");
        };

        ws.onmessage = function (e) {
            console.log("message", e.data);
            term.write(e.data);
        };

        // const attachAddon = new AttachAddon(ws);
        // term.loadAddon(attachAddon);

        term.open(terminalObj);
        term.write(`Welcome to the Vegapunk!\r\n`);
        term.onData((data) => {
            const msg = packageMessage(data)
            ws.send(msg)
        });

    }

    onMount(() => {
        initTerminal();
    });
</script>

<div class="h-full" bind:this={terminalObj}></div>
