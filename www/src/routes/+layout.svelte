<script lang="ts">
  import '../app.css'
  let { children } = $props();
</script>

<svelte:head>
  <title>Take Cover</title>
  <meta name="description" content="Take Cover, a bevy jam submission" />
  <meta
    name="keywords"
    content="Bevy, bevyjam, bevy jam, bevy jam 5"
  />
  <meta name="author" content="co-authored by Alessio Marchi, Mauro Bellinzona">
  <meta name="robots" content="index, follow" />
  <meta name="og:type" content="website" />
  <meta name="og:title" content="Take Cover" />
</svelte:head>

{@render children()}
