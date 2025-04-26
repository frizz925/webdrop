<script lang="ts">
	import type { FileContent } from '$lib/models';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import Content, { type PartialProps } from './Content.svelte';
	import { faDownload } from '@fortawesome/free-solid-svg-icons';

	export interface Props extends PartialProps {
		src: string;
		name: string;
		children: any;
	}

	const { sid, object, src, name, children, onDelete }: Props = $props();
</script>

<Content {sid} {object} {onDelete}>
	<div>
		{@render children()}
	</div>
	<div class="flex h-8 items-center px-4">
		<a href={src} class="text-accent flex cursor-pointer items-center text-sm" download>
			<FontAwesomeIcon icon={faDownload} />
			<div class="pl-2">{name}</div>
		</a>
	</div>
</Content>
