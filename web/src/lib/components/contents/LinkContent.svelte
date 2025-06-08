<script lang="ts">
	import type { FileContent, LinkContent } from '$lib/models';
	import { faDownload, faLink } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		content: LinkContent | FileContent;
		download?: boolean;
	}

	const { object: obj, getFileUrl, content, download, onDelete }: Props = $props();

	const url = (content as LinkContent).url || getFileUrl(obj, content);
	const title = (content as LinkContent).title || (content as FileContent).name || url;
</script>

<Content object={obj} {getFileUrl} {onDelete}>
	<div class="flex items-center justify-start overflow-hidden px-4 pt-4">
		<div class="text-sm">
			<FontAwesomeIcon icon={download ? faDownload : faLink} />
		</div>
		<div class="link text-accent ml-2">
			{#if download}
				<a href={url} target="_blank" download>{title}</a>
			{:else}
				<a href={url} target="_blank">{title}</a>
			{/if}
		</div>
	</div>
</Content>

<style>
	.link {
		display: block;
		overflow: hidden;
		text-overflow: ellipsis;
	}
</style>
