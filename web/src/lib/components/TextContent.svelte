<script lang="ts">
	import type { TextContent } from '$lib/models';
	import Content from './Content.svelte';

	const { content, timestamp }: { content: TextContent; timestamp: Date } = $props();
</script>

<Content {timestamp}>
	<div class="px-4 pt-4">{content.data}</div>
</Content>
