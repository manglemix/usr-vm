<script lang=ts>
	import { browser } from "$app/environment";
	import { untrack } from "svelte";

    let name = $state("");

    const DAYS = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    let inSoftware = $state(false);
    let inMechanical = $state(false);
    let inElectrical = $state(false);
    let inSystems = $state(false);
    let inSocial = $state(false);
    let inAdmin = $state(false);

    let availabilities: string[][] = $state.raw([]);
    let teams: Record<string, string[]> = $state({});

    $effect(() => {
        if (teams.Software !== undefined) {
            inSoftware = teams.Software.includes(name);
        }
        if (teams.Mechanical !== undefined) {
            inMechanical = teams.Mechanical.includes(name);
        }
        if (teams.Electrical !== undefined) {
            inElectrical = teams.Electrical.includes(name);
        }
        if (teams.Systems !== undefined) {
            inSystems = teams.Systems.includes(name);
        }
        if (teams.Social !== undefined) {
            inSocial = teams.Social.includes(name);
        }
        if (teams.Admin !== undefined) {
            inAdmin = teams.Admin.includes(name);
        }
    });
    $effect(() => {
        if (inSoftware) {
            if (untrack(() => teams.Software === undefined)) {
                teams.Software = [name];
            } else if (!untrack(() => teams.Software).includes(name)) {
                untrack(() => teams.Software).push(name);
            }
        } else {
            teams.Software = untrack(() => teams.Software)?.filter((n) => n !== name) ?? [];
        }
    });
    $effect(() => {
        if (inMechanical) {
            if (untrack(() => teams.Mechanical === undefined)) {
                teams.Mechanical = [name];
            } else if (!untrack(() => teams.Mechanical).includes(name)) {
                untrack(() => teams.Mechanical).push(name);
            }
        } else {
            teams.Mechanical = untrack(() => teams.Mechanical)?.filter((n) => n !== name) ?? [];
        }
    });
    $effect(() => {
        if (inElectrical) {
            if (untrack(() => teams.Electrical === undefined)) {
                teams.Electrical = [name];
            } else if (!untrack(() => teams.Electrical).includes(name)) {
                untrack(() => teams.Electrical).push(name);
            }
        } else {
            teams.Electrical = untrack(() => teams.Electrical)?.filter((n) => n !== name) ?? [];
        }
    });
    $effect(() => {
        if (inSystems) {
            if (untrack(() => teams.Systems === undefined)) {
                teams.Systems = [name];
            } else if (!untrack(() => teams.Systems).includes(name)) {
                untrack(() => teams.Systems).push(name);
            }
        } else {
            teams.Systems = untrack(() => teams.Systems)?.filter((n) => n !== name) ?? [];
        }
    });
    $effect(() => {
        if (inSocial) {
            if (untrack(() => teams.Social === undefined)) {
                teams.Social = [name];
            } else if (!untrack(() => teams.Social).includes(name)) {
                untrack(() => teams.Social).push(name);
            }
        } else {
            teams.Social = untrack(() => teams.Social)?.filter((n) => n !== name) ?? [];
        }
    });
    $effect(() => {
        if (inAdmin) {
            if (untrack(() => teams.Admin === undefined)) {
                teams.Admin = [name];
            } else if (!untrack(() => teams.Admin).includes(name)) {
                untrack(() => teams.Admin).push(name);
            }
        } else {
            teams.Admin = untrack(() => teams.Admin)?.filter((n) => n !== name) ?? [];
        }
    });

    async function refreshSchedule() {
        const response = await fetch("http://127.0.0.1/api/scheduler/get/schedule");
        const body = await response.json();
        availabilities = body.availabilities;
        teams = body.teams;
    }

    if (browser) {
        refreshSchedule();
    }
</script>

<input bind:value={name} placeholder="Name" />

<section>
    <label>
        <input type="checkbox" bind:checked={inSoftware} />
        Software
    </label>
    <label>
        <input type="checkbox" bind:checked={inMechanical} />
        Mechanical
    </label>
    <label>
        <input type="checkbox" bind:checked={inElectrical} />
        Electrical
    </label>
    <label>
        <input type="checkbox" bind:checked={inSystems} />
        Systems
    </label>
    <label>
        <input type="checkbox" bind:checked={inSocial} />
        Social
    </label>
    <label>
        <input type="checkbox" bind:checked={inAdmin} />
        Admin
    </label>
</section>

<table>
    <thead>
        <tr>
            {#each DAYS as day}
                <th>{day}</th>
            {/each}
        </tr>
    </thead>
    <tbody>
        {#each { length: 32 } as i}
            <tr>
                {#each DAYS as _, j}
                <td>
                    A
                </td>
                {/each}
            </tr>
        {/each}
    </tbody>
</table>

<style>
    th {
        background-color: darkgray;
    }
    td {
        background-color: lightgray;
    }
    th, td {
        padding: 0.2rem;
        border: 1px solid black;
    }
</style>