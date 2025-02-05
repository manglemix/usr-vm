<script lang="ts">
	import { base } from '$app/paths';
	let { children } = $props();
</script>

<header class="fixed top-0 w-full p-2 shadow">
	<nav class="flex flex-row items-center gap-4">
		<a href={`${base}`} aria-label="Home">
			<enhanced:img src="$lib/icon.png" alt="Utah Student Robotics" class="logo" />
		</a>
		<a href={`${base}/scheduler`}>Scheduler</a>
		<a href={`${base}/attendance`}>Attendance</a>
		<!-- <a href={`${base}/manifest`}>Manifest</a> -->
	</nav>
</header>

<div class="mt-20"></div>
{@render children()}

<style>
	.logo {
		width: 3rem;
		height: auto;
	}

	header {
		background-color: darkgray;
	}

	header nav a {
		font-size: 1.25rem;
		font-weight: 600;
	}
</style>
