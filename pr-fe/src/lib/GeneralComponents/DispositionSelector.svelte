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
      <div>✔️ Add</div>
    </button>
  </div>
  <div>
    {#each dispositions as d, i}
      <div class="disposition">
        <span>{d}</span>
        <button class="remove-button" on:click={removeDispositionAtIndex(i)}>
          <div>❌</div>
        </button>
      </div>
    {/each}
  </div>
</div>


<style>
  .remove-button {
    border-radius: 16px;
    line-height: 17px;
  }

  .disposition {
    display: inline;
    margin-left: 8px;
  }
</style>