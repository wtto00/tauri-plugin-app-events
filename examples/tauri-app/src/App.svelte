<script>
import Greet from "./lib/Greet.svelte";
import {
	onResume,
	onPause,
	onBackKeyDown,
	onMenuKeyDown,
	onSearchKeyDown,
	onVolumeDownKeyDown,
	onVolumeUpKeyDown,
} from "tauri-plugin-app-events-api";
import { listen } from "@tauri-apps/api/event";

let response = "";

listen("js-log", (event) => {
	response += `log from Rust: ${event.payload}<br>`;
});

function updateResponse(returnValue) {
	response += `[${new Date().toLocaleTimeString()}] ${typeof returnValue === "string" ? returnValue : JSON.stringify(returnValue)}<br>`;
}

let hasListened = true;
function clearAll() {
	onResume();
	onPause();
	onBackKeyDown();
	onMenuKeyDown();
	onSearchKeyDown();
	onVolumeDownKeyDown();
	onVolumeUpKeyDown();
	response = "";
	hasListened = false;
}

function addAll() {
	onResume(() => {
		updateResponse("App resume");
	});
	onPause(() => {
		updateResponse("App pause");
	});
	onBackKeyDown(() => {
		updateResponse("Back key triggered");
		return false;
	});
	onMenuKeyDown(() => {
		updateResponse("Menu key triggered");
		return false;
	});
	onSearchKeyDown(() => {
		updateResponse("Seach key triggered");
		return false;
	});
	onVolumeDownKeyDown(() => {
		updateResponse("Volume down key triggered");
		return false;
	});
	onVolumeUpKeyDown(() => {
		updateResponse("Volume up key triggered");
		return false;
	});
	hasListened = true;
}
addAll();
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div>
    <button on:click="{hasListened ? clearAll : addAll}">{hasListened ? 'Clear' : 'Add'} listener</button>
    <div>{@html response}</div>
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
