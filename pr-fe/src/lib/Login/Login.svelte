<script lang="ts">
import { useLocation, useNavigate } from "svelte-navigator";

  import { userStore, loggedInUserStore } from "../../utils/stores";
  import PrSelect from "../GeneralComponents/PrSelect.svelte";

  const navigate = useNavigate();
  const location = useLocation();

  $: userList = $userStore.users.map(u => ({ id: u._id, label: u.name }));
  let selectedUser: string = "";

  const onLogin = () => {
    const u = $userStore.users.find(e => e._id === selectedUser) || null;

    if (u === null) {
      return;
    }
    $loggedInUserStore = u;
    const from = ($location.state && $location.state.from) || "/";
    navigate(from, { replace: true });
  }
</script>

<div>
  <div>Login as</div>
  <div>
    <PrSelect
      id="login-user-list"
      data={userList}
      bind:value={selectedUser}
    />
  </div>
  <button on:click={onLogin} disabled={selectedUser === ""}>
    <div>🔑 Login</div>
  </button>
</div>
