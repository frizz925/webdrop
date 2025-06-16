<script lang="ts">
	import type { FileContent, LinkContent } from '$lib/models';
	import { faDownload, faLink } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import type { Menu } from '../DropdownMenu.svelte';
	import Content, { type PartialProps } from './Content.svelte';
	import { copyToClipboard } from './utils';

	interface Props extends PartialProps {
		link?: string;
		content: LinkContent | FileContent;
		download?: boolean;
	}

	const {
		link: optLink,
		object,
		content,
		download,
		copyMenu: optCopyMenu,
		onDelete
	}: Props = $props();

	const link = optLink || (content as LinkContent).url;
	const title = (content as LinkContent).title || (content as FileContent).name || link;

	const copyMenu: Menu = optCopyMenu || {
		label: 'Copy URL',
		icon: faLink,
		onClick: () => copyToClipboard(link, 'URL'),
		hidden: !object.mime.startsWith('text/')
	};
</script>

<Content {object} {copyMenu} fileUrl={link} {onDelete}>
	<div class="flex items-center justify-start overflow-hidden px-4 pt-4">
		<div class="text-sm">
			<FontAwesomeIcon icon={download ? faDownload : faLink} />
		</div>
		<div class="link text-accent ml-2">
			{#if download}
				<a href={link} target="_blank" download>{title}</a>
			{:else}
				<a href={link} target="_blank">{title}</a>
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
