<script lang="ts">
  import { eventStore } from '../../utils/stores';
  import Event from "./Event.svelte";
</script>

<main>
  <div class="container">
    {#each $eventStore.events as e}
      <Event data={e} />
    {/each}
  </div>
</main>

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
