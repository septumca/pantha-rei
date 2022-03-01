<script lang="ts">
import type { EventData } from "src/types/prtypes.type";

  import { deleteEvent } from "../../utils/services";
  import { eventStore } from '../../utils/stores';

  export let data: EventData;

  const onDelete = async () => {
    const _r = await deleteEvent(data._id);
    eventStore.update(d => ({ ...d, events: d.events.filter(e => e._id !== data._id)}));
  }
</script>

<main>
  <div class="card">
    <button on:click={onDelete} class="delete-button">Delete</button>
    {data.name}
  </div>
</main>

<style>

  .delete-button {
    position: absolute;
    top: 2px;
    right: 2px;
    display: flex;
    padding: 0px;
    cursor: pointer;
  }

  .card {
    position: relative;
    box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
    transition: 0.3s;
    padding: 4px 8px;
    border-radius: 8px;
    border: 2px solid #3a3a3a;
  }
</style>
