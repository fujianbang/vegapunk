<script>
    import {Terminal} from "@xterm/xterm";
    import {FitAddon} from "@xterm/addon-fit";
    import {onMount} from "svelte";
    import "@xterm/xterm/css/xterm.css";
    import HeaderTabs from "./HeaderTabs.svelte";


    /**
     * @type {HTMLElement}
     */
    let terminalObj;

    function initTerminal() {
        // terminalObj = document.getElementById("terminal");

        if (!terminalObj) {
            alert("No terminal found");
            return;
        }

        const term = new Terminal({
            cursorBlink: true,
            fontSize: 14,
            theme: {
                foreground: "red", //字体
                background: "#1e1e1e", //背景色
            },
        });
        // addon
        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        fitAddon.fit();

        term.open(terminalObj);
        term.write(`Welcome to the Vegapunk!`);
        term.onData((data) => {
            console.log(data);
            term.write(data);
        });
    }

    onMount(() => {
        initTerminal();
    });
</script>

<div class="h-full">
    <HeaderTabs/>
    <div class="h-full" bind:this={terminalObj}></div>
</div>
