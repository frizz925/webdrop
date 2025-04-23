<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<div class="bg-color fixed top-0 left-0 flex h-12 w-full items-center justify-center border-b px-4">
	<h1 class="text-xl font-bold">WebDrop</h1>
</div>
<div class="mt-12">
	{@render children()}
</div>
