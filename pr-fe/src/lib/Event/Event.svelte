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

<main>
  <div class="card">
    {#if includeUser}
      <span>🙋‍♂️ Participating</span>
      <button on:click={removeParticipant}>✖️ Leave</button>
    {:else}
      <button on:click={addParticipant}>🎈 Join</button>
    {/if}
    <button on:click={onDelete} class="delete-button">🗑️ Delete</button>
    <div>{data.name}</div>
    <div>Participants: <pre>{JSON.stringify(data.participants)}</pre></div>
  </div>
</main>

<style>

  .delete-button {
    position: absolute;
    top: 2px;
    right: 2px;
    display: flex;
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
