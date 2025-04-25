<script lang="ts">
	import type { LinkContent } from '$lib/models';
	import { faDownload, faLink } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		content: LinkContent;
		download?: boolean;
	}

	const { sid, object, content, download = false, onDelete }: Props = $props();
</script>

<Content {sid} {object} {onDelete}>
	<div class="flex items-center justify-start px-4 pt-4">
		<div class="text-sm">
			<FontAwesomeIcon icon={download ? faDownload : faLink} />
		</div>
		<a href={content.url} target="_blank" class="text-accent ml-2" {download}>
			{content.title || content.url}
		</a>
	</div>
</Content>
