<script lang="ts">
  import { createEvent } from "../../utils/services";
  import { eventStore } from '../../utils/stores';
import type { EventData } from "src/types/prtypes.type";

  let name: string = "";

  const onCreate = async () => {
    let newEvent: EventData = await createEvent(name);
    eventStore.update(d => ({ ...d, events: [...d.events, newEvent] }));
    name = "";
  }
</script>

<div>
  <div>
    <input placeholder="Event name" bind:value={name} >
  </div>
  <button on:click={onCreate} disabled={name === ""}>
    <div>Create new event</div>
  </button>

</div>
