<script lang=ts>
	import { PUBLIC_API_ENDPOINT } from '$env/static/public';
</script>

<!-- This iframe prevents redirection -->
<!-- svelte-ignore a11y_missing_attribute -->
<iframe name="dummyframe" id="dummyframe" style="display: none;"></iframe>

<form method="POST" action="{PUBLIC_API_ENDPOINT}/api/attendance/add/attendance" target="dummyframe">
	<label>
		uID
		<input name="uid" placeholder="u1234567" pattern="u[0-9]+" />
	</label>
	<button onclick={() => {
        alert("Checked In");
    }}>Check in</button>
</form>

<style>
    form {
        display: flex;
        flex-direction: column;
        align-items: start;
        margin: 2rem;
    }
    form label {
        display: flex;
        flex-direction: column;
        margin-bottom: 1rem;
        font-weight: bold;
    }
    form input {
        padding: 0.5rem;
        font-weight: normal;
    }
    form button {
        padding: 0.5rem;
        background-color: lightcoral;
        border: 1px solid black;
        border-radius: 0.25rem;
    }
</style>