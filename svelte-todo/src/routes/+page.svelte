<script>
	import Todo from '../components/Todo.svelte';

	let todoList = [
		{ title: 'first todo', id: 1 },
		{ title: 'second todo', id: 2 },
		{ title: 'third todo', id: 3 },
		{ title: 'fourth todo', id: 4 }
	];

	let addTodoValue = '';
	let lastId = todoList.length;

	function addTodo() {
		lastId++;
		todoList.push({ title: addTodoValue, id: lastId });
		todoList = todoList
		addTodoValue = '';
	}

	function deleteTodo(event) {
		const todoInd = todoList.findIndex((td) => td.id === event.detail.id);
		todoList.splice(todoInd, 1);
		todoList = todoList;
	}
</script>

<div class="bg-black text-white h-screen w-screen p-20">
	<h1 class="text-5xl">Svelte Todo</h1>

	<div class="flex border h-20 justify-around items-center p-5 m-5 bg-slate-50 rounded-lg">
		<input
			class="text-black px-5 h-full w-full items-center flex border-b-4 border-blue-400"
			bind:value={addTodoValue}
		/>
		<button
			on:click={addTodo}
			class={`${false ? 'bg-yellow-500' : 'bg-green-500'} h-full w-40 mx-5 rounded-full`}
			>Add Todo</button
		>
	</div>
	<div class="bg-slate-500 p-5 mt-10">
		{#each todoList as { title, id } (id)}
			<Todo on:deleteMe={deleteTodo} {title} {id} />
		{/each}
	</div>
</div>
