<script lang="ts">
  import init from "../assets/take-cover/take-cover";

  let canvas: HTMLCanvasElement | null = $state(null)

  $effect(() => {
    canvas?.focus();
    init();
  });
</script>

<div class="w-full h-full">
  <canvas bind:this={canvas} class="focus:outline-none" width="100%" height="100%" id="take-cover"></canvas>
</div>
