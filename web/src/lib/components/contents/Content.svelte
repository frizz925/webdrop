<script lang="ts">
	import type { FileObject, ObjectID } from '$lib/models';
	import { faCode, faEllipsisV, faLink, faTrash } from '@fortawesome/free-solid-svg-icons';
	import { format, formatDistanceToNowStrict } from 'date-fns';
	import { onMount, type Snippet } from 'svelte';

	import { createMarkdownURL } from '$lib/utils';
	import IconButton from '../buttons/IconButton.svelte';
	import DropdownMenu, { type Menu } from '../DropdownMenu.svelte';
	import { copyToClipboard } from '../utils';

	export interface PartialProps {
		object: FileObject;
		children?: Snippet;
		copyMenus?: Menu[];
		fileURL?: string;
		filename?: string;
		onDelete?: (oid: ObjectID) => void;
	}

	const { object: obj, children, copyMenus, fileURL, filename, onDelete }: PartialProps = $props();
	const { id, timestamp, content } = obj;
	const datetime = format(timestamp, 'yyyy-MM-dd HH:mm:ss');
	const isTextContent = content.kind !== 'file';

	let timestampShown = $state(false);
	const toggleTimestamp = () => {
		timestampShown = !timestampShown;
	};

	let dropdownShown = $state(false);
	const toggleDropdown = () => {
		dropdownShown = !dropdownShown;
	};

	let elapsed = $state('Just now');
	const updateElapsed = () => {
		elapsed = formatDistanceToNowStrict(timestamp) + ' ago';
	};

	const createLink = () => {
		if (!fileURL) return;
		const parts = fileURL.split('?', 2);
		const url = new URL(window.location.toString());
		url.pathname = parts[0];
		url.search = parts[1] || '';
		return url;
	};

	const copyLink = () => {
		const url = createLink();
		if (!url) return;
		copyToClipboard(url.toString(), 'Object URL');
	};

	const copyMarkdownLink = () => {
		const url = createLink();
		if (!url) return;
		const markdown = createMarkdownURL(url, filename || url.toString());
		copyToClipboard(markdown, 'Markdown URL');
	};

	const menuList: Menu[] = [
		...(copyMenus || []),
		{
			label: 'Copy Object URL',
			icon: faLink,
			onClick: copyLink,
			hidden: isTextContent || !fileURL
		},
		{
			label: 'Copy Markdown URL',
			icon: faCode,
			onClick: copyMarkdownLink,
			hidden: isTextContent || !fileURL
		},
		{
			label: 'Delete Object',
			icon: faTrash,
			onClick: () => onDelete && onDelete(id),
			color: 'red',
			hidden: !onDelete
		}
	].filter((menu) => !!menu);

	onMount(() => {
		updateElapsed();
		const interval = setInterval(updateElapsed, 5000);
		return () => clearInterval(interval);
	});
</script>

<div class="border-b">
	{@render children?.()}
	<div class="text-sub mb-2 flex items-center justify-start pl-4 pr-2 text-sm">
		<div class="grow">
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="inline-block cursor-pointer"
				onclick={toggleTimestamp}
				onkeypress={toggleTimestamp}
			>
				<span class={!timestampShown ? 'inline' : 'hidden'}>
					{elapsed}
				</span>
				<span class={timestampShown ? 'inline' : 'hidden'}>
					{datetime}
				</span>
			</div>
		</div>
		<DropdownMenu bind:shown={dropdownShown} {menuList}>
			<IconButton icon={faEllipsisV} size="xs" onClick={toggleDropdown} />
		</DropdownMenu>
	</div>
</div>
