<script lang="ts">
	import { base } from '$app/paths';
</script>

<div class="flex flex-col items-center">
	<h1 class="mb-8 mt-8 text-center text-5xl font-bold">Utah Student Robotics<br />Web Apps</h1>
	<enhanced:img src="$lib/icon.png" alt="Utah Student Robotics" class="hero-image" />
	<section class="mt-8 flex flex-col gap-4">
		<a class="app-card scheduler" href={`${base}/scheduler`}>
			<h2>Scheduler</h2>
			<p>A unified schedule for Utah Student Robotics</p>
		</a>
		<!-- <a class="app-card manifest" href={`${base}/manifest`}>
			<h2>Manifest</h2>
			<p>Internal tracking of orders and inventory</p>
		</a> -->
	</section>
</div>

<svelte:head>
	<title>USR Web Apps</title>
</svelte:head>

<style>
	.app-card {
		border-radius: 1rem;
		padding: 1rem;
		border: 2px solid black;
	}
	.app-card h2 {
		font-size: 1.2rem;
		font-weight: bold;
	}
	.scheduler {
		background-color: aquamarine;
	}
	/* .manifest {
		background-color: cornflowerblue;
	} */
	.hero-image {
		width: 20rem;
		height: auto;
	}
</style>
