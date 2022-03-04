<script lang="ts">
  import { setLoggedUser } from "../../utils/auth";
  import { useLocation, useNavigate } from "svelte-navigator";
  import { userStore } from "../../utils/stores";
  import PrSelect from "../GeneralComponents/PrSelect.svelte";
  import { useFocus } from "svelte-navigator";

  const registerFocus = useFocus();

  const navigate = useNavigate();
  const location = useLocation();

  $: userList = $userStore.users.map(u => ({ id: u._id, label: u.name }));
  let selectedUser: string = "";

  const onLogin = () => {
    const u = $userStore.users.find(e => e._id === selectedUser) || null;

    if (u === null) {
      return;
    }

    setLoggedUser(u);
    const from = ($location.state && $location.state.from) || "/";
    navigate(from, { replace: true });
  }
</script>

<div>
  <div use:registerFocus>Login as</div>
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
