<script>
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import { onMount } from "svelte";

  let terminalObj;

  function initTerminal() {
    terminalObj = document.getElementById("terminal");

    console.log(terminalObj);

    if (!terminalObj) {
      alert("No terminal found");
      return;
    }

    var term = new Terminal();
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(terminalObj);
    fitAddon.fit();
    term.write("Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ");
  }

  onMount(() => {
    initTerminal();
  });
</script>

<div id="terminal"></div>
