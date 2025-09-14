<script lang="ts">
  import { core } from "$platform";

  let users_state = $state<{id: string, name: string}[]>([]);
  let show_error = $state(false);
  let error_message = $state("");
  let runtimeName = $state(core.getRuntimeName());

  async function load_users(event: Event) {
    event.preventDefault();
    let users = await core.executeCommand("users-list", {});
    if(users.success) {
        users_state = users.value;
    } else {
        show_error = true;
        error_message = users.error;
    }
  }
</script>

<main class="container">
  <h1>Welcome DSOT (using {runtimeName})</h1>
  <button onclick={load_users}>Load Users</button>
  {#if show_error}
    <p style="color: red">Error loading users</p>
    <p style="color: red">{error_message}</p>
  {/if}
  {#if users_state.length > 0}
    <ul>
      {#each users_state as user}
        <li>{user.id} ({user.name})</li>
      {/each}
    </ul>
  {/if}
</main>

<style>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}


h1 {
  text-align: center;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
