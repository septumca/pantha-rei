<script lang="ts">
  import type { UserData } from 'src/types/prtypes.type';
  import { updateUser } from '../../utils/services';
  import DispositionSelector from '../GeneralComponents/DispositionSelector.svelte';
  import { getLoggedUser, setLoggedUser } from '../../utils/auth';
  import { useFocus } from "svelte-navigator";

  const registerFocus = useFocus();

  let user: UserData = getLoggedUser();

  const onUpdate = async () => {
    await updateUser(user._id, user);
    setLoggedUser(user);
  }
</script>

<main>
  <div>
    Name: <input use:registerFocus placeholder="User name" bind:value={user.name} >
  </div>
  <div>Dispositions: <DispositionSelector bind:dispositions={user.dispositions} /></div>
  <div>
    <button on:click={onUpdate} disabled={user.name === ""}>💾 Update</button>
  </div>
</main>

<style>
  div {
    margin-bottom: 8px;
  }
</style>