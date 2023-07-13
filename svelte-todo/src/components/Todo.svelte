<script>
	import { createEventDispatcher } from 'svelte';

	export let title;
	export let id;

	let disabled = true;
	const dispatch = createEventDispatcher();

	function doEdit() {
		disabled = !disabled;
	}

	function deleteMe() {
		dispatch('deleteMe', { id });
	}
</script>

<div class="flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg">
	<input
		class="text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400"
		value={title}
		{disabled}
	/>
	<button
		on:click={doEdit}
		class={`${disabled ? 'bg-yellow-500' : 'bg-green-500'} h-full w-40 mx-5 rounded-full`}
	>
		{disabled ? 'Edit' : 'Done'}
	</button>
	<button on:click={deleteMe} class="bg-red-500 h-full w-40 mx-5 rounded-full">Delete</button>
</div>
