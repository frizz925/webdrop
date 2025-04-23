<script lang="ts">
	import Content from './Content.svelte';

	const { content, timestamp }: { content: string; timestamp: Date } = $props();
</script>

<Content {timestamp}>
	<div class="px-4 pt-4">{content}</div>
</Content>
