<script lang="ts">
  import { onMount } from 'svelte';

  let gamepads:any[] = []

	onMount(async() => {
    setInterval(() => {
      gamepads = navigator.getGamepads().filter(g => g !== null)
    }, 100);
  })

  window.ongamepadconnected = (e) =>
    console.log("ongamepadconnected", e);
  
  window.ongamepaddisconnected = (e) =>
    console.log("ongamepaddisconnected", e);
</script>

<main class="container">
  {#each gamepads as gamepad}
  <div>
    <h1>{gamepad.index} {gamepad.id}</h1>
    <h2>Axes</h2>
    <pre>{gamepad.axes}</pre>
    <h2>Buttons</h2>
    <pre>{gamepad.buttons.map(b => b.value)}</pre>
    <br/>
  </div>
  {/each}
</main>