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
            payload: {
                data: message
            },
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

        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.open(terminalObj);
        fitAddon.fit();

        const ws = new WebSocket("ws://localhost:1234/ws/a706fe14-d770-4e74-813f-1bc592476eda/nginx");
        const attachAddon = new AttachAddon(ws, {bidirectional: true});
        term.loadAddon(attachAddon);

        term.write(`Welcome to the Vegapunk!\r\n`);
        term.onData((data) => {
            const msg = packageMessage(data)
            ws.send(msg)
        });
        term.onResize((size) => {
            const msg = JSON.stringify({
                type: "resize",
                payload: {
                    width: size.cols,
                    height: size.rows
                }
            })
            ws.send(msg)
        })

        window.addEventListener("resize", resizeScreen)

        function resizeScreen() {
            fitAddon.fit()
        }
    }

    onMount(() => {
        initTerminal();
    });
</script>

<div class="h-full" bind:this={terminalObj}></div>
