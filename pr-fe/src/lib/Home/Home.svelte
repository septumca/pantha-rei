<script lang="ts">
  import type { UserData } from 'src/types/prtypes.type';
  import { loggedInUserStore } from '../../utils/stores';
  import { updateUser } from '../../utils/services';
  import DispositionSelector from '../GeneralComponents/DispositionSelector.svelte';

  let userData: UserData = $loggedInUserStore;

  const onUpdate = async () => {
    await updateUser(userData._id, userData);
    loggedInUserStore.set({ ...userData });
  }
</script>

<main>
  <div>Name: </div>
  <div>
    <input placeholder="User name" bind:value={userData.name} >
  </div>
  <div>Dispositions: </div>
  <DispositionSelector bind:dispositions={userData.dispositions} />
  <div>
    <button on:click={onUpdate} disabled={userData.name === ""}>💾 Update</button>
  </div>
</main>
