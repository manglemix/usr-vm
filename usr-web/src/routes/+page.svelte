<div class="flex flex-col items-center">
    <h1 class="text-center mt-8 mb-8 text-5xl font-bold">Utah Student Robotics<br>Web Apps</h1>
    <enhanced:img src="$lib/icon.png" alt="Utah Student Robotics" class="hero-image"/>
    <section class="flex flex-col gap-4 mt-8">
        <a class="app-card scheduler" href="/scheduler">
            <h2>Scheduler</h2>
            <p>
                A unified schedule for Utah Student Robotics
            </p>
        </a>
        <a class="app-card manifest" href="/manifest">
            <h2>Manifest</h2>
            <p>
                Internal tracking of orders and inventory
            </p>
        </a>
    </section>
</div>

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
    .manifest {
        background-color: cornflowerblue;
    }
	.hero-image {
		width: 20rem;
		height: auto;
	}
</style>