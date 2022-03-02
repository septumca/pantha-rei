<script lang="ts">
  export let id: string = "";
  export let data: Array<{ id: string, label: string }>;
  export let value: string = "";
</script>

<select name="select-{id}" id={id} bind:value={value}>
  <option value="">-</option>
  {#each data as d}
    <option value={d.id}>{d.label}</option>
  {/each}
</select>