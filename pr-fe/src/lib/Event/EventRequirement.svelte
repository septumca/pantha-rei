<script lang="ts">
import { fullfillRequirement, unfullfillRequirement } from "../../utils/services";
import { setFullfillmentToEvent } from "../../utils/stores";

  import type { Requirement, UserData } from "../../types/prtypes.type";
  import { getLoggedUser } from "../../utils/auth";

  export let data: Requirement;
  export let eventId: string;
  let user: UserData = getLoggedUser();

  $: hasBeenFullfilled = data.fullfilled_by !== null;
  $: isFullfilledByUser = hasBeenFullfilled && data.fullfilled_by._id === user._id;
  $: canBeAssigned = hasBeenFullfilled === false && user.dispositions.includes(data.name);

  const fullFill = async () => {
    await fullfillRequirement(eventId, data.name, user);
    setFullfillmentToEvent(eventId, data.name, { _id: user._id, name: user.name });
  };

  const unfullFill = async () => {
    await unfullfillRequirement(eventId, data.name, user._id);
    setFullfillmentToEvent(eventId, data.name, null);
  };
</script>

<div>
  <span>
    {#if hasBeenFullfilled}🟢{:else}🔴{/if}
    {data.name}
  </span>
  {#if hasBeenFullfilled}
    <span>
      {data.fullfilled_by.name}
      {#if isFullfilledByUser}
        <button on:click={unfullFill}>🛑 Sing off</button>
      {/if}
    </span>
  {:else if canBeAssigned}
    <button on:click={fullFill}>💡 Sing up</button>
  {/if}
</div>

<style>
  div {
    margin: 4px 0px;
  }
</style>
