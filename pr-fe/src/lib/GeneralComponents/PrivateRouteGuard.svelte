<script>
  import { useNavigate, useLocation } from "svelte-navigator";
  import { loggedInUserStore } from "../../utils/stores";

  const navigate = useNavigate();
  const location = useLocation();

  $: if ($loggedInUserStore === null) {
    navigate("/login", {
      state: { from: $location.pathname },
      replace: true,
    });
  }
</script>

{#if $loggedInUserStore !== null}
  <slot />
{/if}