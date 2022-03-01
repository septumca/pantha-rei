<script lang="ts">
import type { UserData } from "src/types/prtypes.type";

  import { deleteUser } from "../../utils/services";
  import { userStore } from "../../utils/stores";

  export let data: UserData;

  const onDelete = async () => {
    const _r = await deleteUser(data._id);
    userStore.update(d => ({ ...d, users: d.users.filter(e => e._id !== data._id)}));
  }
</script>

<main>
  <div class="card">
    <button on:click={onDelete} class="delete-button">Delete</button>
    <div>{data.name}</div>
    <div>{data.dispositions.join(', ')}</div>
  </div>
</main>

<style>
  .delete-button {
    position: absolute;
    top: 2px;
    right: 2px;
    display: flex;
    padding: 0px;
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
