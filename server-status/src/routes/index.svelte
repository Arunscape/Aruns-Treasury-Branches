<script>
    import {onMount} from 'svelte'
    let data;

    onMount(async () => {
    const res = await fetch(`https://status.minecraft.woosaree.xyz/api/info`);
    data = await res.json();
    console.log(data)
  });
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<div>{JSON.stringify(data)}</div>
