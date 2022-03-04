<script lang="ts">
  import { eventStore } from '../../utils/stores';
  import Event from "./Event.svelte";
  import { useFocus } from "svelte-navigator";

  const registerFocus = useFocus();
</script>

<div use:registerFocus class="container">
  {#each $eventStore.events as e}
    <Event data={e} />
  {/each}
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 1fr;
    margin: 0px 8px;
    column-gap: 16px;
    row-gap: 16px;
  }

  @media (min-width: 40em) {
    .container {
      grid-template-columns: 1fr 1fr;
    }
  }

  @media (min-width: 64em) {
    .container {
      grid-template-columns: 1fr 1fr 1fr;
    }
  }
</style>
