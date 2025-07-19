<script lang="ts">
  import { enhance } from "$app/forms";
  import ImageInput from "$lib/elements/ImageInput.svelte";
  import type { PageData } from "./$types";


  type Props = {
    data: PageData;
  };
  let { data }: Props = $props();

  let image = data.image;

  let alt = $state(image?.alt_text);
</script>

<form method="post" use:enhance enctype="multipart/form-data">
  <ImageInput bind:alt src="/images/{image?.file_name}" readonly/>
  <textarea
    name="new_alt_desc"
    id="ipt-alt-desc"
    required
    placeholder="Alt Text"
    bind:value={alt}
  ></textarea>
  <div class="action-btns">
    <a href="/" aria-label="Cancelar">
      <input type="button" value="Cancelar" />
    </a>
    <input type="submit" value="Gravar" />
  </div>
</form>

<style>
  form {
    display: flex;
    flex-direction: column;
    width: 50vw;
    margin-inline: auto;
    margin-block: 0 1em;
    gap: 0.5em;
    flex: 1;

    textarea {
      flex: 1;

      border: none;
      appearance: none;
      background: #f2f2f2;
      padding: 12px;
      border-radius: 3px;
      font-size: 14px;
      resize: none;
    }

    .action-btns {
      display: flex;
      justify-content: space-between;

      input {
        padding: 0.25em 0.5em;
        border: none;
        appearance: none;
        background: #0068ff;
        color: white;
        border-radius: 5px;
      }
    }
  }
</style>
