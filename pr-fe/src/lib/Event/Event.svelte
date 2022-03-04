<script lang="ts">
  import type { EventData, UserData } from "../../types/prtypes.type";
  import { getLoggedUser } from "../../utils/auth";
  import { addUserToEvent, removeUserFromEvent } from "../../utils/services";
  import { deleteEvent } from "../../utils/services";
  import { removeEvent, updateEvent } from '../../utils/stores';
  import EventRequirement from "./EventRequirement.svelte";

  export let data: EventData;
  let user: UserData = getLoggedUser();

  $: includeUser = data.participants.some(d => d._id === user._id);

  const onDelete = async () => {
    await deleteEvent(data._id);
    removeEvent(data._id);
  }

  const removeParticipant = async () => {
    await removeUserFromEvent(data._id, user._id);
    data.participants = data.participants.filter(u => u._id !== user._id);
    updateEvent(data);
  }

  const addParticipant = async () => {
    await addUserToEvent(data._id, user);
    data.participants = [ ...data.participants, user ];
    updateEvent(data);
  }
</script>

<div class="card">
  <div class="left-controls">
    {#if includeUser}
      <div><span>🙋‍♂️ Participating</span></div>
      <div><button on:click={removeParticipant}>✖️ Leave</button></div>
    {:else}
      <button on:click={addParticipant}>🎈 Join</button>
    {/if}
  </div>
  <div class="right-controls"><button on:click={onDelete}>🗑️ Delete</button></div>
  <div class="title">{data.name}</div>
  <div class="content">
    <div>
      Participants: {data.participants.length}
    </div>
    <div>
      {data.description}
    </div>
    <div>
      {#each data.requirements as d}
        <EventRequirement eventId={data._id} data={d} />
      {/each}
    </div>
  </div>
</div>

<style>
  .left-controls {
    grid-area: lc;
  }
  .title {
    grid-area: tl;
  }
  .right-controls {
    grid-area: rc;
    text-align: right;
  }
  .content {
    grid-area: ct;
  }

  .card {
    box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
    transition: 0.3s;
    padding: 4px 8px;
    border-radius: 8px;
    border: 2px solid #3a3a3a;
    display: grid;
    row-gap: 8px;
    grid-template-columns: 1fr 1fr;
    grid-template-areas:
      "tl tl"
      "lc rc"
      "ct ct"
      "ct ct"
  }
</style>
