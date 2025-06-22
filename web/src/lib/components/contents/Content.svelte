<script lang="ts">
	import type { FileObject, ObjectID } from '$lib/models';
	import { faEllipsisV, faLink, faTrash } from '@fortawesome/free-solid-svg-icons';
	import { format, formatDistanceToNowStrict } from 'date-fns';
	import { onMount, type Snippet } from 'svelte';

	import IconButton from '../buttons/IconButton.svelte';
	import DropdownMenu, { type Menu } from '../DropdownMenu.svelte';
	import { copyToClipboard } from '../utils';

	export interface PartialProps {
		object: FileObject;
		children?: Snippet;
		copyMenu?: Menu;
		fileURL?: string;
		onDelete?: (oid: ObjectID) => void;
	}

	const { object: obj, children, copyMenu, fileURL: fileURL, onDelete }: PartialProps = $props();
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

	const copyLink = () => {
		if (!fileURL) return;
		const url = new URL(window.location.toString());
		url.pathname = fileURL;
		copyToClipboard(url.toString(), 'Object URL');
	};

	const menuList: Menu[] = [
		copyMenu,
		{
			label: 'Copy Object URL',
			icon: faLink,
			onClick: copyLink,
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
