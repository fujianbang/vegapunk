<script>
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import { onMount } from "svelte";

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

    var term = new Terminal({
      cursorBlink: true,
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

<div bind:this={terminalObj}></div>
