<script lang="ts">
  type Props = {
    alt?: string;
    src?: string;
    readonly?: boolean;
  };

  let { alt = $bindable(""), src = "#", readonly = false }: Props = $props();

  let files = $state<FileList>();
  let imgElement = $state<HTMLImageElement>();
  let inputElement = $state<HTMLInputElement>();
  let dragging = $state(false);
  let error = $state(false);

  $effect(() => {
    if (!files || !imgElement || !inputElement) return;

    const [file] = files;

    if (file) {
      console.log();
      error = !file.type.match(/image\/.*/g);

      if (error) {
        inputElement.setCustomValidity("Invalid file type");
        imgElement.src = "#";
      } else {
        inputElement.setCustomValidity("");
        imgElement.src = URL.createObjectURL(file);
      }
    }
  });

  function onDrop(ev: DragEvent) {
    if (readonly) {
      return;
    }

    console.log("File(s) dropped");

    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();

    if (ev?.dataTransfer?.items) {
      console.log(ev.dataTransfer.items);
    }
    if (ev?.dataTransfer?.files) {
      files = ev?.dataTransfer.files;
    }

    dragging = false;
  }

  function dragOverHandler(ev: DragEvent) {
    if (readonly) {
      return;
    }
    console.log("File(s) in drop zone");

    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();
  }
</script>

<div class="img-input-container">
  <img
    {src}
    {alt}
    class:dragging
    class:error
    bind:this={imgElement}
    ondrop={onDrop}
    ondragover={dragOverHandler}
    ondragenter={() => (dragging = !readonly && true)}
    ondragleave={() => (dragging = !readonly && false)}
  />
  {#if !readonly}
    <label for="img-input" class="img-input-lbl">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        height="24px"
        viewBox="0 -960 960 960"
        width="24px"
        fill="#e8eaed"
      >
        <path
          d="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z"
        />
      </svg>
      <input
        id="img-input"
        accept="image/jpg, image/webp, image/png,"
        type="file"
        name="image"
        bind:files
        required
        bind:this={inputElement}
      />
      <span>Enviar Imagem</span>
    </label>
  {/if}
</div>

<style lang="scss">
  .img-input-container {
    display: flex;
    flex-direction: column;
    gap: 0.5em;
  }

  img {
    border: black solid 1px;
    border-radius: 5px;
    min-height: 400px;
    max-height: 400px;
    width: fit-content;
    margin-inline: auto;

    object-fit: scale-down;

    &[src="#"] {
      width: 100%;
    }

    &.error {
      border: #aa2200 solid 3px;
    }

    &.dragging {
      border: #0068ff dotted 3px;
    }
  }

  .img-input-lbl {
    display: flex;
    max-width: fit-content;
    margin-inline: auto 0;
    padding: 0.25em 0.5em;
    background: #0068ff;
    color: white;
    border-radius: 5px;
    gap: 0.25em;

    svg {
      fill: currentColor;
      width: 24px;
      height: 24px;
      display: block;
      flex-shrink: 0;
    }

    input[type="file"] {
      max-width: 0;
    }
  }
</style>
