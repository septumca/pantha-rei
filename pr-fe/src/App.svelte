<script lang="ts">
  import { onMount } from "svelte";
  import { eventStore, refDataStore, userStore } from "./utils/stores";
  import NewEvent from "./lib/Event/NewEvent.svelte";
  import { getEvents, getUsers, getReferenceData } from "./utils/services";
  import NewUser from "./lib/User/NewUser.svelte";
  import type { EventData, ReferenceData, UserData } from "./types/prtypes.type";
  import { Router, Link, Route } from "svelte-navigator";
  import Login from "./lib/Login/Login.svelte";
  import PrivateRoute from "./lib/GeneralComponents/PrivateRoute.svelte";
  import Home from "./lib/Home/Home.svelte";
  import Events from "./lib/Event/Events.svelte";
  import Logout from "./lib/Logout/Logout.svelte";
  import Users from "./lib/User/Users.svelte";

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
    <div>
      <Route path="login">
        <Login />
      </Route>

      <PrivateRoute path="/*">
        <div class="navbar">
          <nav>
            <Link to="/">Home</Link>
            <Link to="/events">Events</Link>
            <Link to="/users">Users</Link>
          </nav>
          <Logout />
        </div>
        <Route path="/">
          <Home />
        </Route>
        <Route path="events">
          <Events />
          <Link to="new">New event</Link>
        </Route>
        <Route path="events/new">
          <NewEvent />
        </Route>
        <Route path="users/*">
          <Users />
          <Link to="users/new">New user</Link>
          <Route path="new">
            <NewUser />
          </Route>
        </Route>
      </PrivateRoute>

    </div>
  </Router>



</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  .navbar {
    border-bottom: 1px solid rgb(0, 134, 187);
    padding-bottom: 8px;
    margin-bottom: 8px;
  }

  :global(input) {
    padding: 4px;
  }

  :global(select) {
    padding: 4px;
  }

  :global(button) {
    cursor: pointer;
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

</style>
