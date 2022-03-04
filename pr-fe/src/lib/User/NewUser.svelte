<script lang="ts">
  import { createUser } from "../../utils/services";
  import { userStore } from '../../utils/stores';
  import type { UserData } from "src/types/prtypes.type";
  import DispositionSelector from "../GeneralComponents/DispositionSelector.svelte";
  import { useFocus } from "svelte-navigator";

  const registerFocus = useFocus();

  let name: string = "";
  let dispositions: Array<string> = [];

  const onCreate = async () => {
    let user: UserData = await createUser(name, dispositions);
    userStore.update(d => ({ ...d, users: [...d.users, user] }));
    name = "";
    dispositions = [];
  }
</script>

<main>
  <div>
    <input use:registerFocus placeholder="User name" bind:value={name} >
  </div>
  <DispositionSelector bind:dispositions={dispositions} />
  <button on:click={onCreate} disabled={name === ""}>
    <div>🙂✔️ Create new user</div>
  </button>
</main>
