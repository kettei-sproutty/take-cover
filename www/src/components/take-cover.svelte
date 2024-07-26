<script lang="ts">
  import init from "../assets/take-cover/take-cover";
  import clsx from "clsx";

  type GameStatus = "idle" | "loading" | "running";

  let gameStatus = $state<GameStatus>("idle");

  const onGameStart = async () => {
    gameStatus = "loading";
    await init().finally(() => {
      gameStatus = "running";
    });
  };
</script>

<div aria-busy={gameStatus === "loading"} class="w-full h-full">
  {#if gameStatus === "idle"}
    <div class="flex flex-col items-center justify-center h-full">
      <button
        type="button"
        onclick={onGameStart}
        class="bg-primary text-primary-100 hover:bg-primary-900 hover:text-primary-200 px-4 py-2 rounded focus:outline-none"
        >Start Game</button
      >
    </div>
  {:else if gameStatus === "loading"}
    <div class="flex flex-col items-center justify-center h-full">
      <svg
        class="animate-spin size-5 text-primary-100"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
      >
        <circle
          class="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          stroke-width="4"
        ></circle>
        <path
          class="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
        ></path>
      </svg>
    </div>
  {/if}
  <canvas
    class={clsx("focus:outline-none", {
      hidden: gameStatus !== "running",
    })}
    width="100%"
    height="100%"
    id="take-cover"
  ></canvas>
</div>
