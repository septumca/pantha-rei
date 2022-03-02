<script lang="ts">
  import { onMount } from "svelte";
  import { eventStore, refDataStore, userStore, loggedInUserStore } from "./utils/stores";
  import Event from "./lib/Event/Event.svelte";
  import NewEvent from "./lib/Event/NewEvent.svelte";
  import { getEvents, getUsers, getReferenceData } from "./utils/services";
  import NewUser from "./lib/User/NewUser.svelte";
  import User from "./lib/User/User.svelte";
  import type { EventData, ReferenceData, UserData } from "./types/prtypes.type";
  import { Router, Link, Route } from "svelte-navigator";
  import Login from "./lib/Login/Login.svelte";
  import PrivateRoute from "./lib/GeneralComponents/PrivateRoute.svelte";

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
  <Router>
    <nav>
      <Link to="/">Home</Link>
      <Link to="/events">Events</Link>
      <Link to="/users">Users</Link>
    </nav>
    <div>
      <Route path="login">
        <Login />
      </Route>

      <PrivateRoute path="/">
        HOME
      </PrivateRoute>
      <PrivateRoute path="/events">
        <div class="container">
          {#each $eventStore.events as e}
            <Event data={e} />
          {/each}
        </div>
        <Link to="/events/new">New event</Link>
      </PrivateRoute>
      <PrivateRoute path="/events/new">
        <NewEvent />
      </PrivateRoute>
      <PrivateRoute path="/users">
        <div class="container">
          {#each $userStore.users as u}
            <User data={u} />
          {/each}
        </div>
        <Link to="/users/new">New user</Link>
      </PrivateRoute>
      <PrivateRoute path="/users/new">
        <NewUser />
      </PrivateRoute>
    </div>
  </Router>



</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  :global(input) {
    padding: 4px;
  }

  :global(select) {
    padding: 4px;
  }

  :global(button) {
    background-color: rgb(0, 134, 187);
    transition: background-color 0.3s;
    border: none;
    color: azure;
    padding: 4px;
    border-radius: 4px;
  }

  :global(button:disabled) {
    background-color: rgba(0, 134, 187, 0.5);
    transition: background-color 0.3s;
  }

  :global(button:hover:not(:disabled)) {
    background-color: rgb(0, 164, 230);
    transition: background-color 0.3s;
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
