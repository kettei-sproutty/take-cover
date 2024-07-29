<script lang="ts">
  import TakeCover from "../components/take-cover.svelte";
  const authors = ["kettei-sproutty", "BackupMiles"];
</script>

<header
  class="bg-primary-800 text-primary-100 h-10 flex items-center justify-between px-2 gap-2"
>
  <h1 class="inline">Bevy Jam 5 | Take Cover</h1>
</header>
<section class="w-full h-[calc(100dvh-5rem)]">
  <TakeCover />
</section>
<footer
  class="bg-primary-800 text-primary-100 h-10 flex items-center justify-center px-2 gap-2"
>
  <ul class="inline-flex gap-1 text-xs">
    <li>Made with 🤍 by:</li>
    {#each authors as author}
      <li class="hover:underline">
        <a href={`https://github.com/${author}`}>{author}</a>
      </li>
      <span aria-hidden="true" class="last:hidden inline">/</span>
    {/each}
  </ul>
</footer>
