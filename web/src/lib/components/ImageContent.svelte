<script lang="ts">
	import Content from './Content.svelte';

	const { src, name, timestamp }: { src: string; name: string; timestamp: Date } = $props();
</script>

<Content {timestamp}>
	<img {src} alt={name} class="w-full" />
</Content>
