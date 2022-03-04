<script lang="ts">
  import { userStore } from '../../utils/stores';
  import User from "./User.svelte";
  import { useFocus } from "svelte-navigator";

  const registerFocus = useFocus();
</script>

<div use:registerFocus class="container">
  {#each $userStore.users as u}
    <User data={u} />
  {/each}
</div>

<style>
  .container {
    text-align: center;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    margin: 16px;
    column-gap: 16px;
    row-gap: 16px;
  }
</style>
