<script lang="ts">
	import { browser } from '$app/environment';
	import { untrack } from 'svelte';
	import { PUBLIC_API_ENDPOINT } from '$env/static/public';
	import { TeamQuery, type Team } from '$lib';
	import { parse } from 'svelte/compiler';
	import { SvelteSet } from 'svelte/reactivity';

	let name = $state('');

	const DAYS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

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
	function uploadTeams() {
		if (name.length === 0) {
			return;
		}
		const teams: Team[] = [];
		if (inSoftware) {
			teams.push('Software');
		}
		if (inMechanical) {
			teams.push('Mechanical');
		}
		if (inElectrical) {
			teams.push('Electrical');
		}
		if (inSystems) {
			teams.push('Systems');
		}
		if (inSocial) {
			teams.push('Social');
		}
		if (inAdmin) {
			teams.push('Admin');
		}
		fetch(`${PUBLIC_API_ENDPOINT}/api/scheduler/set/team`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				name,
				teams
			})
		});
	}
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
		uploadTeams();
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
		uploadTeams();
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
		uploadTeams();
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
		uploadTeams();
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
		uploadTeams();
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
		uploadTeams();
	});

	async function refreshSchedule() {
		const response = await fetch(`${PUBLIC_API_ENDPOINT}/api/scheduler/get/schedule`);
		const body = await response.json();
		availabilities = body.availabilities;
		teams = body.teams;
	}

	if (browser) {
		refreshSchedule();
		window.addEventListener('pointerup', async () => {
			if (!updateDrag) {
				return;
			}
			if (name.length === 0) {
				return;
			}
			const times = [];

			for (let y = Math.min(dragStartY, dragEndY); y <= Math.max(dragStartY, dragEndY); y++) {
				for (let x = Math.min(dragStartX, dragEndX); x <= Math.max(dragStartX, dragEndX); x++) {
					times.push(y + x * 10 * 4);
				}
			}

			const body = JSON.stringify({
				name,
				times
			});

			if (deleting) {
				const response = await fetch(`${PUBLIC_API_ENDPOINT}/api/scheduler/del/schedule`, {
					method: 'DELETE',
					headers: {
						'Content-Type': 'application/json'
					},
					body
				});
				if (response.ok) {
					for (const i of times) {
						availabilities[i] = availabilities[i].filter((n) => n !== name);
					}
					availabilities = availabilities.map((a) => a);
				}
			} else {
				const response = await fetch(`${PUBLIC_API_ENDPOINT}/api/scheduler/add/schedule`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body
				});
				if (response.ok) {
					for (const i of times) {
						if (!availabilities[i].includes(name)) {
							availabilities[i].push(name);
						}
					}
					availabilities = availabilities.map((a) => a);
				}
			}
			updateDrag = false;
		});
	}

	let tabIndex = $state(0);
	let updateDrag = $state(false);
	let deleting = $state(false);
	let dragStartX = $state(0);
	let dragStartY = $state(0);
	let dragEndX = $state(0);
	let dragEndY = $state(0);

	function isPositionInsideDrag(x: number, y: number) {
		return (
			Math.min(dragStartX, dragEndX) <= x &&
			x <= Math.max(dragStartX, dragEndX) &&
			Math.min(dragStartY, dragEndY) <= y &&
			y <= Math.max(dragStartY, dragEndY)
		);
	}

	function isPositionInsideAvailabilities(x: number, y: number, name: string) {
		return availabilities[y + x * 10 * 4]?.includes(name) ?? false;
	}

	function isUpdateCellGreen(x: number, y: number) {
		if (updateDrag) {
			if (isPositionInsideDrag(x, y)) {
				if (deleting) {
					return false;
				} else {
					return true;
				}
			}
		}
		return isPositionInsideAvailabilities(x, y, name);
	}

	function timeString(y: number) {
		const hour = Math.floor(y / 4) + 9;
		let hourCorrected;
		if (hour === 12) {
			hourCorrected = 12;
		} else {
			hourCorrected = hour % 12;
		}
		const minutes = (y % 4) * 15;
		return `${hourCorrected}:${minutes === 0 ? '00' : minutes}${hour < 12 ? 'a' : 'p'}`;
	}

	let advancedFilterQuery = $state('');
	let advancedFilterParseError = $state('');
	let maxPresent = $state(new SvelteSet<string>());
	let filterFn: (x: number, y: number) => Set<string> = $state(() => new Set());
	let selectedCellPeople: Set<string> | null = $state(null);
	let selectedCell: [number, number] | null = $state(null);

	function filterFnFromQuery(query: TeamQuery) {
		untrack(() => {
			maxPresent.clear();
		});

		filterFn = (x, y) => {
			const newTeams: Record<string, string[]> = {};
			const newNames: Set<string> = new Set();

			for (const [team, names] of Object.entries(teams)) {
				const newSubteam = names.filter(
					(name) => availabilities[y + x * 10 * 4]?.includes(name) ?? false
				);
				newTeams[team] = newSubteam;
				for (const name of newSubteam) {
					newNames.add(name);
				}
			}

			const out = query.evaluate(newTeams, newNames);
			untrack(() => {
				maxPresent = new SvelteSet(maxPresent.union(out));
			});
			return out;
		};
	}

	let simpleQuerySoftware = $state(false);
	let simpleQueryMechanical = $state(false);
	let simpleQueryElectrical = $state(false);
	let simpleQuerySystems = $state(false);
	let simpleQuerySocial = $state(false);
	let simpleQueryAdmin = $state(false);

	$effect(() => {
		if (tabIndex !== 1) {
			return;
		}
		const teams = [];
		if (simpleQuerySoftware) {
			teams.push('Software');
		}
		if (simpleQueryMechanical) {
			teams.push('Mechanical');
		}
		if (simpleQueryElectrical) {
			teams.push('Electrical');
		}
		if (simpleQuerySystems) {
			teams.push('Systems');
		}
		if (simpleQuerySocial) {
			teams.push('Social');
		}
		if (simpleQueryAdmin) {
			teams.push('Admin');
		}
		if (teams.length === 0) {
			filterFnFromQuery(
				TeamQuery.parse('Software or Mechanical or Electrical or Systems or Social or Admin')
			);
		} else {
			filterFnFromQuery(TeamQuery.parse(teams.join(' or ')));
		}
	});
</script>

<svelte:head>
	<title>USR Scheduler</title>
</svelte:head>

<section id="schedule-operations" class="mb-4 ml-4 mr-4">
	<div id="schedule-tabs" class="flex flex-row">
		<button
			onclick={() => {
				tabIndex = 0;
			}}
			id={tabIndex === 0 ? 'selected-operation' : ''}
		>
			Update
		</button>
		<button
			onclick={() => {
				tabIndex = 1;
				selectedCellPeople = null;
				selectedCell = null;
			}}
			id={tabIndex === 1 ? 'selected-operation' : ''}
		>
			Simple Filter
		</button>
	</div>
	<section id="schedule-operations-content" class="flex flex-col">
		{#if tabIndex === 0}
			<input bind:value={name} placeholder="Your Name" />

			<section class="flex flex-col">
				<label>
					<input type="checkbox" bind:checked={inSoftware} disabled={name.length === 0} />
					Software
				</label>
				<label>
					<input type="checkbox" bind:checked={inMechanical} disabled={name.length === 0} />
					Mechanical
				</label>
				<label>
					<input type="checkbox" bind:checked={inElectrical} disabled={name.length === 0} />
					Electrical
				</label>
				<label>
					<input type="checkbox" bind:checked={inSystems} disabled={name.length === 0} />
					Systems
				</label>
				<label>
					<input type="checkbox" bind:checked={inSocial} disabled={name.length === 0} />
					Social
				</label>
				<label>
					<input type="checkbox" bind:checked={inAdmin} disabled={name.length === 0} />
					Admin
				</label>
			</section>
		{:else if tabIndex === 1}
			<section class="flex flex-col">
				<label>
					<input type="checkbox" bind:checked={simpleQuerySoftware} />
					Software
				</label>
				<label>
					<input type="checkbox" bind:checked={simpleQueryMechanical} />
					Mechanical
				</label>
				<label>
					<input type="checkbox" bind:checked={simpleQueryElectrical} />
					Electrical
				</label>
				<label>
					<input type="checkbox" bind:checked={simpleQuerySystems} />
					Systems
				</label>
				<label>
					<input type="checkbox" bind:checked={simpleQuerySocial} />
					Social
				</label>
				<label>
					<input type="checkbox" bind:checked={simpleQueryAdmin} />
					Admin
				</label>
			</section>
		{/if}
	</section>
</section>

<div class="m-4 flex flex-row-reverse flex-wrap justify-end gap-4">
	{#if tabIndex !== 0}
		<section class="flex flex-row flex-wrap gap-4">
			{#if selectedCellPeople !== null}
				{#each selectedCellPeople as name}
					<div>{name}</div>
				{/each}
			{:else}
				{#each maxPresent as name}
					<div>{name}</div>
				{/each}
			{/if}
		</section>
	{/if}

	<table>
		<thead>
			<tr>
				<th></th>
				{#each DAYS as day}
					<th>{day}</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each { length: 40 } as _, y}
				<tr>
					<td>{timeString(y)}</td>
					{#each DAYS as _, x}
						{#if tabIndex === 0}
							<td
								class="schedule-cell unscrollable"
								style:--p={isUpdateCellGreen(x, y) ? '100%' : '0%'}
								onpointerdown={(event) => {
									if (name === '') {
										return;
									}
									event.currentTarget.releasePointerCapture(event.pointerId);
									deleting = isPositionInsideAvailabilities(x, y, name);
									updateDrag = true;
									dragStartX = x;
									dragStartY = y;
									dragEndX = x;
									dragEndY = y;
								}}
								onpointerenter={(event) => {
									if (updateDrag) {
										event.stopPropagation();
										dragEndX = x;
										dragEndY = y;
									}
								}}
							>
							</td>
						{:else}
							{#snippet scheduleCell(people: Set<string>)}
								<td
									class="schedule-cell"
									style:--p={`${maxPresent.size === 0 ? 0 : (people.size / maxPresent.size) * 100}%`}
									title={`${people.size} / ${maxPresent.size === 0 ? 1 : maxPresent.size}`}
									onpointerenter={() => {
										selectedCellPeople = people;
										selectedCell = [x, y];
									}}
									id={selectedCell !== null && selectedCell[0] === x && selectedCell[1] === y
										? 'selected-cell'
										: ''}
								>
								</td>
							{/snippet}
							{@render scheduleCell(filterFn(x, y))}
						{/if}
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	th {
		background-color: darkgray;
		padding: 0.2rem;
		min-width: 3rem;
	}
	.schedule-cell {
		background-color: color-mix(in oklab, darkgray, green var(--p));
		padding: 0;
	}
	.unscrollable {
		touch-action: none;
	}
	td {
		background-color: lightgray;
		padding-right: 0.2rem;
	}
	th,
	td {
		border: 1px solid black;
		/* user-drag: none; */
		-webkit-user-drag: none;
		user-select: none;
		-moz-user-select: none;
		-webkit-user-select: none;
		-ms-user-select: none;
	}
	#schedule-tabs > button {
		background-color: darkgray;
		padding: 0.2rem;
		border: 1px solid black;
	}
	#schedule-operations #selected-operation {
		background-color: lightgray;
	}
	#selected-cell {
		background-color: aqua;
	}
	#schedule-operations-content {
		background-color: lightgray;
	}
</style>
