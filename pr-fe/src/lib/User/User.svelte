<script lang="ts">
import type { UserData } from "src/types/prtypes.type";

  import { deleteUser } from "../../utils/services";
  import { removeUser } from "../../utils/stores";

  export let data: UserData;

  const onDelete = async () => {
    const _r = await deleteUser(data._id);
    removeUser(data._id);
  }
</script>

<main>
  <div class="card">
    <div class="title">{data.name}</div>
    <div class="right-controls"><button on:click={onDelete} class="delete-button">🗑️ Delete</button></div>
    <div class="content">{data.dispositions.join(', ')}</div>
  </div>
</main>

<style>
  .title {
    grid-area: tl;
  }
  .right-controls {
    grid-area: lc;
    text-align: right;
  }
  .content {
    grid-area: ct;
  }

  .card {
    box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
    transition: 0.3s;
    padding: 4px 8px;
    border-radius: 8px;
    border: 2px solid #3a3a3a;
    display: grid;
    gap: 4px;
    grid-template-columns: 3fr 1fr;
    grid-template-areas:
      "tl lc"
      "ct ct"
  }
</style>
