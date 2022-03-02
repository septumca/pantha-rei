<script lang="ts">
  import { refDataStore } from '../../utils/stores';
  import PrSelect from "../GeneralComponents/PrSelect.svelte";

  export let dispositions: Array<string> = [];

  let newDisposition: string = "";
  let dispositionList: Array<string> = $refDataStore.dispositions;
  $: filteredDispositionList = dispositionList.filter(d => !dispositions.includes(d)).map(d => ({ id: d, label: d }));

  const onAddDisposition = () => {
    dispositions = [...dispositions, newDisposition];
    newDisposition = "";
  }

  const removeDispositionAtIndex = index => () => {
    dispositions = dispositions.filter((_e, i) => i !== index);
  }
</script>

<div>
  <div>
    <PrSelect
      id="disposition-list"
      data={filteredDispositionList}
      bind:value={newDisposition}
    />
    <button on:click={onAddDisposition} disabled={newDisposition === ""}>
      <div>✔️ Add new disposition</div>
    </button>
  </div>
  <div>
    {#each dispositions as d, i}
      <div class="disposition">
        <span>{d}</span>
        <button on:click={removeDispositionAtIndex(i)}>
          <div>❌ Remove</div>
        </button>
      </div>

    {/each}
  </div>
</div>


<style>
  .disposition {
    display: inline;
    padding-left: 8px;
  }
</style>