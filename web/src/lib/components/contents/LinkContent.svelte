<script lang="ts">
	import type { FileContent, LinkContent, SessionID } from '$lib/models';
	import { getFileURL } from '$lib/utils';
	import { faDownload, faLink } from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	import type { Menu } from '../DropdownMenu.svelte';
	import { copyToClipboard } from '../utils';
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		sid?: SessionID;
		link?: string;
		content: LinkContent | FileContent;
		download?: boolean;
	}

	const {
		sid,
		link: optLink,
		object: obj,
		content,
		download,
		copyMenu: optCopyMenu,
		onDelete
	}: Props = $props();

	const getLinkFromContent = () => {
		if (content.kind === 'link') return (content as LinkContent).url;
		else if (content.kind === 'file') {
			if (sid) return getFileURL(sid, obj, content as FileContent);
			else throw new Error("Can't get file URL without session ID");
		}
		throw new Error(`Can't get link for content kind ${content.kind}`);
	};

	const link = optLink || getLinkFromContent();
	const title = (content as LinkContent).title || (content as FileContent).name || link;
	const copyMenu: Menu = optCopyMenu || {
		label: 'Copy URL',
		icon: faLink,
		onClick: () => copyToClipboard(link, 'URL'),
		hidden: obj.content.kind === 'file'
	};
</script>

<Content object={obj} {copyMenu} fileURL={link} {onDelete}>
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
