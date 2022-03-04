<script lang="ts">
  import type { EventData, UserData } from "../../types/prtypes.type";
  import { getLoggedUser } from "../../utils/auth";
  import { addUserToEvent, removeUserFromEvent } from "../../utils/services";
  import { deleteEvent } from "../../utils/services";
  import { eventStore } from '../../utils/stores';

  export let data: EventData;
  let user: UserData = getLoggedUser();

  $: includeUser = data.participants.some(d => d._id === user._id);

  const onDelete = async () => {
    await deleteEvent(data._id);
    eventStore.update(d => ({ ...d, events: d.events.filter(e => e._id !== data._id)}));
  }

  const removeParticipant = async () => {
    await removeUserFromEvent(data._id, user._id);
    data.participants = data.participants.filter(u => u._id !== user._id);
    eventStore.update(d => ({ ...d, events: d.events.map(e => e._id === data._id ? data : e)}));
  }

  const addParticipant = async () => {
    await addUserToEvent(data._id, user);
    data.participants = [ ...data.participants, user ];
    eventStore.update(d => ({ ...d, events: d.events.map(e => e._id === data._id ? data : e)}));
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
  <div class="content">Participants: {data.participants.map(p => p.name).join(', ') || "None"}</div>
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
    gap: 4px;
    grid-template-columns: 1fr 1fr;
    grid-template-areas:
      "tl tl"
      "lc rc"
      "ct ct"
  }
</style>
