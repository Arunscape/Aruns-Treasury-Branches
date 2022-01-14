<script>
	import { onMount } from 'svelte';

	// onMount(async () => {
	// 	const res = await fetch(`https://status.minecraft.woosaree.xyz/api/info`, {
	// 		mode: 'no-cors' // 'cors' by default
	// 	});
	// 	data = await res.json();
	// 	console.log(data);
	// });

	const data = fetch('https://status.minecraft.woosaree.xyz/api/info').then((res) => res.json());
</script>

<h1>minecraft.woosaree.xyz status page</h1>
<!-- <div>{JSON.stringify(data)}</div> -->
{#await data}
	<div>loading...</div>
{:then d}
	<div>
		<img src={d.favicon} />
		<div>{d.num_online}/{d.max_players} Players online</div>
		<div>
			<div>Players preview</div>
			{#each d.players_sample as p}
				<div>{p.name}</div>
			{/each}
		</div>

		<div>Version: {d.version}</div>
        <div>Description</div>
        <div>{d.description}</div>

		<div>
			<div>Additional info</div>
			<div>
				<div>Ping from {d.ping.from} to minecraft.woosaree.xyz is {d.ping.time}ms</div>
			</div>
		</div>
	</div>
{/await}
