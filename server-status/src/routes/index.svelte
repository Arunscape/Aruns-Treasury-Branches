<script lang="ts">
	import { onMount } from 'svelte';

	type Player = {
		name: string;
		id: string;
	};
	type PingResponse = {
		ping: {
			from: string;
			time: number;
		};
		max_players: number;
		players_sample: Player[];
		favicon: string;
		version: string;
		protocol: string;
		description: string;
		num_online: number;
	};

	const data = fetch('https://mcstatus.arun.gg/api/info').then((res) => res.json());
</script>

<h1 class="changecolour">mc.arun.gg status page</h1>
<!-- <div>{JSON.stringify(data)}</div> -->
{#await data}
	<div>loading...</div>
{:then d}
	<div>
		<img
			src={d.favicon}
			alt="Server favicon. This shows up in the server list when you connect to a multiplayer server in the minecraft client."
		/>
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
				<div>Ping from {d.ping.from} to mc.arun.gg is {d.ping.time}ms</div>
			</div>
		</div>
	</div>
{/await}

<style>
	/* @Nigel the CSS goes here */
	/* this style I added is trash  so pls delete it */
	.changecolour {
		animation: pulse 5s infinite;
	}

	@keyframes pulse {
		0% {
			background-color: #001f3f;
		}
		100% {
			background-color: #ff4136;
		}
	}
</style>
