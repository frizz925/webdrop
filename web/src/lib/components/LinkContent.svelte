<script lang="ts">
	import { faDownload, faLink } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import Content from './Content.svelte';

	interface Props {
		name: string;
		timestamp: Date;
		url: string;
		download?: boolean;
	}

	const { name, timestamp, url, download = false }: Props = $props();
</script>

<Content {timestamp}>
	<div class="flex items-center justify-start px-4 pt-4">
		<div class="text-sm">
			<FontAwesomeIcon icon={download ? faDownload : faLink} />
		</div>
		<a href={url} target="_blank" class="text-accent ml-2" {download}>{name}</a>
	</div>
</Content>
