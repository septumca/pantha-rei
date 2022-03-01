<script lang="ts">
  import { onMount } from "svelte";
  import { eventStore, refDataStore, userStore } from "./utils/stores";
  import Event from "./lib/Event/Event.svelte";
  import NewEvent from "./lib/Event/NewEvent.svelte";
  import { getEvents, getUsers, getReferenceData } from "./utils/services";
  import NewUser from "./lib/User/NewUser.svelte";
  import User from "./lib/User/User.svelte";
  import type { EventData, ReferenceData, UserData } from "./types/prtypes.type";

	let events: Array<EventData> = [];
  let users: Array<UserData> = [];

	eventStore.subscribe(d => {
    events = d.events;
  });
  userStore.subscribe(d => {
    users = d.users;
  });


  onMount(async () => {
    const fe: Promise<Array<EventData>> = getEvents();
    const fu: Promise<Array<UserData>> = getUsers();
    const fr: Promise<ReferenceData> = getReferenceData();

    const [e, u, r] = await Promise.all([fe, fu, fr]);
    eventStore.set({ events: e });
    userStore.set({ users: u });
    refDataStore.set(r);
  });
</script>


<main>
  <div class="container">
    {#each events as e}
      <Event data={e} />
    {/each}
  </div>
  <NewEvent />
  <div class="container">
    {#each users as u}
      <User data={u} />
    {/each}
  </div>
  <NewUser />
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  .container {
    text-align: center;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    margin: 16px;
    column-gap: 16px;
    row-gap: 16px;
  }
</style>
