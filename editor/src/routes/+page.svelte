<script lang="ts">
  import type { PageData } from "./$types";

  type Props = {
    data: PageData;
  };
  let { data }: Props = $props();

  let images = data.images;
</script>

<main>
  <div class="image-list">
    {#if images}
      {#each images as image}
        <div class="img-container">
          <img src="/images/{image.file_name}" alt={image.alt_text} />
          <a href="/edit/{image.descriptor_id}">
            {image.file_name}
          </a>
        </div>
      {/each}
    {/if}
  </div>
  <a href="/new" class="new-image" aria-label="Nova Imagem">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="40px"
      viewBox="0 -960 960 960"
      width="40px"
      fill="#e8eaed"
    >
      <path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z" />
    </svg>
  </a>
</main>

<style lang="scss">
  main {
    display: flex;
    flex: 1;
    margin-inline: 2em;
  }
  .image-list {
    display: grid;
    gap: 1rem;
    grid-template-columns: repeat(7, 1fr);
    grid-auto-rows: min-content;
    margin-inline: auto;

    .img-container {
      font-size: 8pt;
      text-align: center;
      height: fit-content;

      & > img {
        width: 240px;
        height: auto;
        aspect-ratio: 16 /9;
        object-fit: scale-down;
      }
    }
  }

  .new-image {
    display: grid;
    place-items: center;
    height: 60px;
    aspect-ratio: 1/1;
    position: fixed;
    right: 30px;
    bottom: 30px;
    border-radius: 100vw;

    padding: 0.25em 0.5em;
    border: none;
    appearance: none;
    background: #0068ff;
    color: white;

    box-shadow: 2px 2px 4px 0 #00000088;
  }
</style>
