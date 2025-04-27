<script lang="ts">
	import type { FileContent, LinkContent } from '$lib/models';
	import { getFileUrl } from '$lib/utils';
	import { faDownload, faLink } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		content: LinkContent | FileContent;
		download?: boolean;
	}

	const { sid, object: obj, content, download, onDelete }: Props = $props();

	const url = (content as LinkContent).url || getFileUrl(sid, obj, content as FileContent);
	const title = (content as LinkContent).title || (content as FileContent).name || url;
</script>

<Content {sid} object={obj} {onDelete}>
	<div class="flex items-center justify-start px-4 pt-4">
		<div class="text-sm">
			<FontAwesomeIcon icon={download ? faDownload : faLink} />
		</div>
		{#if download}
			<a href={url} target="_blank" class="text-accent ml-2" download>
				{title}
			</a>
		{:else}
			<a href={url} target="_blank" class="text-accent ml-2">
				{title}
			</a>
		{/if}
	</div>
</Content>
