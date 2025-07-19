<script lang="ts">
  import "$lib/style/index.scss";
  let { children } = $props();
</script>

<nav>Daily McGillis Fareed Gallery</nav>
{@render children()}

<style lang="scss">
  nav {
    display: flex;
    height: 4rem;
    place-content: center;
    align-content: center;
    align-items: center;
    justify-content: center;
    background: #4259be;
    box-shadow:
      0 9px 0 0 #e1e9f0,
      0 10px 0 0 black,
      0 21px 0 0 #dbcd72,
      0 22px 1px 0 #000a;
    margin-bottom: 2em;
    flex-shrink: 0;
    color: white;
    font-weight: bold;
    font-size: 18pt;
  }
</style>
