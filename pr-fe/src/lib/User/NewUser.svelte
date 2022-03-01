<script lang="ts">
  import { createUser } from "../../utils/services";
  import { refDataStore, userStore } from '../../utils/stores';
import type { UserData } from "src/types/prtypes.type";

  let name: string = "";
  let new_disposition: string = "";
  let dispositions: Array<string> = [];

  let disposition_list: Array<string> = [];
  $: filtered_disposition_list = disposition_list.filter(d => !dispositions.includes(d));

  refDataStore.subscribe(s => {
		disposition_list = s.dispositions;
	});

  const onCreate = async () => {
    let user: UserData = await createUser(name, dispositions);
    userStore.update(d => ({ ...d, users: [...d.users, user] }));
    name = "";
    dispositions = [];
    new_disposition = "";
  }

  const onAddDisposition = () => {
    dispositions = [...dispositions, new_disposition];
    new_disposition = "";
  }

  const removeDispositionAtIndex = index => () => {
    dispositions = dispositions.filter((_e, i) => i !== index);
  }
</script>

<main>
  <div>
    <input
      placeholder="User name"
      bind:value={name}
    >
  </div>
  <div>
    <select name="dispositions" id="user-dispositions-select" bind:value={new_disposition}>
      <option value="">--Select disposition--</option>
      {#each filtered_disposition_list as d}
        <option value={d}>{d}</option>
      {/each}
    </select>
    <button on:click={onAddDisposition} disabled={new_disposition === ""}>
      <div>Add new disposition</div>
    </button>
  </div>
  <div>
    {#each dispositions as d, i}
      <div class="disposition">
        <span>{d}</span>
        <button on:click={removeDispositionAtIndex(i)} disabled={new_disposition === ""}>
          <div>Remove</div>
        </button>
      </div>

    {/each}
  </div>
  <button on:click={onCreate} disabled={name === ""}>
    <div>Create new user</div>
  </button>
</main>

<style>
  .disposition {
    display: inline;
    padding-left: 8px;
  }
</style>