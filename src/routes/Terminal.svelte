<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { invoke } from "@tauri-apps/api/tauri";

  let terminal: Terminal;
  let connectionId: string;

  async function disconnect() {
    if (connectionId) {
      await invoke("disconnect_ssh", { connectionId });
      connectionId = "";
    }
  }

  onMount(async () => {
    terminal = new Terminal();
    const terminalElement = document.getElementById("terminal");
    if (terminalElement) {
      terminal.open(terminalElement);

      connectionId = await invoke("connect_ssh", {
        host: "192.168.1.1",
        port: 22,
        username: "user",
        password: "pass",
      });

      terminal.onData((data) => {
        invoke("execute_ssh_command", {
          connectionId,
          command: data,
        }).then((response: string) => {
          terminal.write(response);
        });
      });
    }
  });

  onDestroy(async () => {
    await disconnect();
  });
</script>

<div id="terminal"></div>

<style>
  #terminal {
    height: 400px;
    width: 600px;
  }
</style>
