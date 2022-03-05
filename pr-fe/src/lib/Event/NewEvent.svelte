<script lang="ts">
  import { createEvent } from "../../utils/services";
  import { addEvent } from '../../utils/stores';
  import type { EventData } from "src/types/prtypes.type";
  import { useFocus } from "svelte-navigator";
  import DispositionSelector from "../GeneralComponents/DispositionSelector.svelte";

  const registerFocus = useFocus();

  let name: string = "";
  let description: string = "";
  let requirements: Array<string> = [];

  const onCreate = async () => {
    let newEvent: EventData = await createEvent(name, description, requirements);
    addEvent(newEvent);
    name = "";
    description = "";
  }
</script>

<div>
  <div>
    <input use:registerFocus placeholder="Event name" bind:value={name} >
  </div>
  <div>
    <textarea id="event-description" name="event-description" placeholder="Description of an event" rows="4" cols="40" bind:value={description} />
  </div>
  <div>
    <DispositionSelector bind:dispositions={requirements} />
  </div>
  <button on:click={onCreate} disabled={name === ""}>
    <div>🗓️✔️ Create new event</div>
  </button>
</div>
