<script lang="ts">
	import type { TextContent } from '$lib/models';
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		content: TextContent;
	}

	const { object, getFileUrl, content, onDelete }: Props = $props();
</script>

<Content {object} {getFileUrl} {onDelete}>
	<div class="overflow-hidden px-4 pt-4 wrap-anywhere whitespace-pre-wrap">{content.data}</div>
</Content>
