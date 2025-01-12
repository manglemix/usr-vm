<script lang="ts">
	let { children } = $props();
</script>

<header class="fixed top-0 w-full p-2 shadow">
	<nav class="flex flex-row items-center gap-4">
		<enhanced:img src="$lib/icon.png" alt="Utah Student Robotics" class="logo" />
		<a href="/scheduler">Scheduler</a>
		<a href="/manifest">Manifest</a>
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
