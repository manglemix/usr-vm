<script lang="ts">
	import { browser } from '$app/environment';
	import { PUBLIC_API_ENDPOINT } from '$env/static/public';
	import type { Team } from '$lib';

	let hideInStorage = $state(false);
	let selectedOrderId: number | null = $state(null);
	let selectedOrderIndex: number = $state(0);
	let fetching = $state(false);
	let orderOperationOutput = $state('');
	let tabIndex = $state(0);

	$effect(() => {
		if (hideInStorage) {
			if (selectedOrderId !== null && orders[selectedOrderId].status === 'In Storage') {
				selectedOrderId = null;
			}
		}
	});

	interface Order {
		id: number;
		name: string;
		date: string;
		status: 'New' | 'Submitted' | 'Shipped' | 'Delivered' | 'InStorage' | 'In Storage';
		count: number;
		unit_cost: number | string;
		store_in: string;
		team: Team;
		reason: string;
		vendor: string;
		link: string;
	}
	let orders: Order[] = $state([]);

	async function refreshOrders() {
		fetching = true;
		const response = await fetch(`${PUBLIC_API_ENDPOINT}/api/manifest/list/order`);
		selectedOrderId = null;
		orders = await response.json();
		orders = orders.map((order) => {
			order.date = new Date(order.date).toLocaleString('en-US', {
				weekday: 'short',
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			});
			if (order.status === 'InStorage') {
				order.status = 'In Storage';
			}
			order.unit_cost = parseFloat(order.unit_cost as string);
			return order;
		});
		setTimeout(() => {
			fetching = false;
		}, 500);
	}

	if (browser) {
		refreshOrders();
	}

	let pending_order_name = $state('');
	let pending_order_vendor = $state('');
	let pending_order_link = $state('');
	let pending_order_count = $state(0);
	let pending_order_unit_cost = $state(0);
	let pending_order_store_in: string = $state('');
	let pending_order_team: Team | '' = $state('');
	let pending_order_reason = $state('');
	let updated_order_status: Order['status'] | '' = $state('');

	function populatePending() {
		if (selectedOrderId !== null) {
			const order = orders[selectedOrderIndex];
			pending_order_name = order.name;
			pending_order_vendor = order.vendor;
			pending_order_link = order.link;
			pending_order_count = order.count;
			pending_order_unit_cost = order.unit_cost as number;
			pending_order_store_in = order.store_in;
			pending_order_team = order.team;
			pending_order_reason = order.reason;
		}
	}

	function clearPending() {
		pending_order_name = '';
		pending_order_vendor = '';
		pending_order_link = '';
		pending_order_count = 0;
		pending_order_unit_cost = 0;
		pending_order_store_in = '';
		pending_order_team = '';
		pending_order_reason = '';
		updated_order_status = '';
	}
</script>

<svelte:head>
	<title>USR Manifest</title>
</svelte:head>

<section class="m-4 w-min">
	<div class="flex flex-row justify-between">
		<label>
			<input type="checkbox" bind:checked={hideInStorage} />
			Hide "In Storage"
		</label>

		{#if fetching}
			<button disabled> Fetching... </button>
		{:else}
			<button onclick={refreshOrders}> Refresh </button>
		{/if}
	</div>
	<table>
		<thead>
			<tr>
				<th>Name</th>
				<th>Date</th>
				<th>Vendor</th>
				<th>Link</th>
				<th>Status</th>
				<th>Count</th>
				<th>Unit Cost</th>
				<th>Store In</th>
				<th>Team</th>
				<th>Reason</th>
				<th>Subtotal</th>
			</tr>
		</thead>
		<tbody>
			{#each orders as order, i}
				{#if !hideInStorage || order.status !== 'In Storage'}
					<tr
						onclick={() => {
							selectedOrderId = order.id;
							selectedOrderIndex = i;
							if (tabIndex == 1) {
								populatePending();
							}
						}}
						id={selectedOrderId === order.id ? 'selectedOrder' : ''}
					>
						<td class="order-name">{order.name}</td>
						<td class="order-date">{order.date}</td>
						<td class="order-vendor">{order.vendor}</td>
						<td><a href={order.link}>Link</a></td>
						<td class="order-status">{order.status}</td>
						<td>{order.count}</td>
						<td
							>{order.unit_cost.toLocaleString('en-US', { style: 'currency', currency: 'USD' })}</td
						>
						<td>{order.store_in ?? ''}</td>
						<td class="order-team">{order.team}</td>
						<td><p class="order-reason">{order.reason}</p></td>
						<td
							>{(order.count * (order.unit_cost as number)).toLocaleString('en-US', {
								style: 'currency',
								currency: 'USD'
							})}</td
						>
					</tr>
				{/if}
			{/each}
		</tbody>
	</table>
</section>

<section id="order-operations" class="mt-4">
	<div id="order-tabs" class="flex flex-row">
		<button
			onclick={() => {
				orderOperationOutput = '';
				if (tabIndex === 1 && selectedOrderId !== null) {
					clearPending();
				}
				tabIndex = 0;
			}}
			id={tabIndex === 0 ? 'selected-operation' : ''}
		>
			New Order
		</button>
		<button
			onclick={() => {
				orderOperationOutput = '';
				tabIndex = 1;
				populatePending();
			}}
			id={tabIndex === 1 ? 'selected-operation' : ''}
		>
			Change Order
		</button>
		<button
			onclick={() => {
				orderOperationOutput = '';
				tabIndex = 2;
				updated_order_status = '';
			}}
			id={tabIndex === 2 ? 'selected-operation' : ''}
		>
			Update Order
		</button>
		<button
			onclick={() => {
				orderOperationOutput = '';
				tabIndex = 3;
			}}
			id={tabIndex === 3 ? 'selected-operation' : ''}
		>
			Cancel Order
		</button>
	</div>
	{#snippet selectAnOrder()}
		<h2>Select an order</h2>
	{/snippet}
	{#snippet input()}
		<label>
			Item Name*
			<input type="text" bind:value={pending_order_name} placeholder="Item Name" />
		</label>

		<label>
			Vendor*
			<input type="text" bind:value={pending_order_vendor} placeholder="Vendor" />
		</label>

		<label>
			Link*
			<input type="url" bind:value={pending_order_link} placeholder="Link to the store" />
		</label>

		<div class="num-inputs flex flex-row justify-around gap-4">
			<label>
				Count*
				<input type="number" bind:value={pending_order_count} />
			</label>

			<label>
				Unit Cost* (USD)
				<input type="number" bind:value={pending_order_unit_cost} step="0.01" />
			</label>
		</div>

		<label>
			Team*
			<select id="team" bind:value={pending_order_team}>
				<option value="" disabled selected>Select a team</option>
				<option value="Software">Software</option>
				<option value="Mechanical">Mechanical</option>
				<option value="Electrical">Electrical</option>
				<option value="Systems">Systems</option>
				<option value="Social">Social</option>
				<option value="Admin">Admin</option>
			</select>
		</label>

		<label>
			Reason*
			<textarea bind:value={pending_order_reason} placeholder="Reason"></textarea>
		</label>

		<label>
			Store In
			<input
				type="text"
				bind:value={pending_order_store_in}
				placeholder="Where to leave the item"
			/>
		</label>
	{/snippet}
	<section id="order-operations-content" class="flex flex-col gap-4 p-4">
		{#if tabIndex === 0}
			{@render input()}

			<button
				onclick={async () => {
					if (
						pending_order_name.trim() === '' ||
						pending_order_vendor.trim() === '' ||
						pending_order_link.trim() === '' ||
						pending_order_count <= 0 ||
						pending_order_unit_cost <= 0 ||
						pending_order_team.length === 0 ||
						pending_order_reason.trim() === ''
					) {
						orderOperationOutput = 'Please fill in all the required fields';
						return;
					}
					const response = await fetch(`${PUBLIC_API_ENDPOINT}/api/manifest/new/order`, {
						method: 'POST',
						headers: {
							'Content-Type': 'application/json'
						},
						body: JSON.stringify({
							name: pending_order_name,
							vendor: pending_order_vendor,
							link: pending_order_link,
							count: pending_order_count,
							unit_cost: pending_order_unit_cost,
							team: pending_order_team,
							reason: pending_order_reason,
							store_in: pending_order_store_in
						})
					});
					if (response.ok) {
						orderOperationOutput = '';
						refreshOrders();
					} else {
						orderOperationOutput = await response.text();
					}
				}}
			>
				Submit
			</button>
			<output>{orderOperationOutput}</output>
		{:else if tabIndex === 1}
			{#if selectedOrderId === null}
				{@render selectAnOrder()}
			{:else}
				{@render input()}
				<button
					onclick={async () => {
						if (
							pending_order_name.trim() === '' ||
							pending_order_vendor.trim() === '' ||
							pending_order_link.trim() === '' ||
							pending_order_count <= 0 ||
							pending_order_unit_cost <= 0 ||
							pending_order_team.length === 0 ||
							pending_order_reason.trim() === ''
						) {
							orderOperationOutput = 'Please fill in all the required fields';
							return;
						}
						const response = await fetch(`${PUBLIC_API_ENDPOINT}/api/manifest/change/order`, {
							method: 'POST',
							headers: {
								'Content-Type': 'application/json'
							},
							body: JSON.stringify({
								id: selectedOrderId,
								name: pending_order_name,
								vendor: pending_order_vendor,
								link: pending_order_link,
								count: pending_order_count,
								unit_cost: pending_order_unit_cost,
								team: pending_order_team,
								reason: pending_order_reason,
								store_in: pending_order_store_in
							})
						});
						if (response.ok) {
							orderOperationOutput = '';
							refreshOrders();
						} else {
							orderOperationOutput = await response.text();
						}
					}}
				>
					Change Order
				</button>
				<output>{orderOperationOutput}</output>
			{/if}
		{:else if tabIndex === 2}
			{#if selectedOrderId === null}
				{@render selectAnOrder()}
			{:else}
				<label for="order-status">Status*</label>
				<select id="order-status" bind:value={updated_order_status}>
					<option value="" disabled selected>Select a status</option>
					<option value="New">New</option>
					<option value="Submitted">Submitted</option>
					<option value="Shipped">Shipped</option>
					<option value="Delivered">Delivered</option>
					<option value="InStorage">In Storage</option>
				</select>

				<button
					onclick={async () => {
						if (updated_order_status.length === 0) {
							orderOperationOutput = 'Please select a status';
							return;
						}
						const response = await fetch(`${PUBLIC_API_ENDPOINT}/api/manifest/update/order`, {
							method: 'POST',
							headers: {
								'Content-Type': 'application/json'
							},
							body: JSON.stringify({
								id: selectedOrderId,
								status: updated_order_status
							})
						});
						if (response.ok) {
							orderOperationOutput = '';
							refreshOrders();
						} else {
							orderOperationOutput = await response.text();
						}
					}}
				>
					Update Order
				</button>
				<output>{orderOperationOutput}</output>
			{/if}
		{:else if tabIndex === 3}
			{#if selectedOrderId === null}
				{@render selectAnOrder()}
			{:else}
				<button
					onclick={async () => {
						const response = await fetch(`${PUBLIC_API_ENDPOINT}/api/manifest/del/order`, {
							method: 'DELETE',
							headers: {
								'Content-Type': 'application/json'
							},
							body: JSON.stringify({
								id: selectedOrderId
							})
						});
						if (response.ok) {
							orderOperationOutput = '';
							refreshOrders();
						} else {
							orderOperationOutput = await response.text();
						}
					}}
				>
					Cancel Order
				</button>
				<output>{orderOperationOutput}</output>
			{/if}
		{/if}
	</section>
</section>

<style>
	th {
		background-color: darkgray;
	}
	td {
		background-color: lightgray;
	}
	th,
	td {
		border: 1px solid black;
		padding: 0.5em;
	}
	tr:hover td {
		background-color: lightblue;
	}
	#selectedOrder td {
		background-color: lightgreen;
	}
	.order-reason {
		max-width: 15rem;
		min-width: 10rem;
	}
	.order-name {
		min-width: 5rem;
	}
	.order-date {
		min-width: 11rem;
	}
	.order-status {
		min-width: 6rem;
	}
	#order-tabs > button {
		background-color: darkgray;
		padding: 0.2rem;
		border: 1px solid black;
	}
	#order-operations #selected-operation {
		background-color: lightgray;
	}
	#order-operations-content > label {
		display: flex;
		flex-direction: column;
	}
	#order-operations-content {
		background-color: lightgray;
	}
	#order-operations-content > button {
		background-color: darkgray;
		padding: 0.2rem;
		border: 1px solid black;
	}
	.num-inputs input {
		width: 6rem;
	}
	.num-inputs label {
		display: flex;
		flex-direction: row;
		gap: 0.5rem;
		align-items: center;
		text-align: end;
	}
</style>
