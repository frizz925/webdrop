<script lang="ts">
	import type { Content, FileObject, LinkContent, ObjectID, TextContent } from '$lib/models';
	import { faClipboard, faEllipsisV, faLink, faTrash } from '@fortawesome/free-solid-svg-icons';
	import { format, formatDistanceToNowStrict } from 'date-fns';
	import { onMount, type Snippet } from 'svelte';
	import IconButton from '../buttons/IconButton.svelte';
	import DropdownMenu, { type Menu } from '../DropdownMenu.svelte';
	import { toastState } from '../state.svelte';

	export interface PartialProps {
		object: FileObject;
		getFileUrl: (obj: FileObject, content: Content) => string;
		children?: Snippet;
		onDelete?: (oid: ObjectID) => void;
	}

	interface Props extends PartialProps {
		children?: Snippet;
	}

	const { object: obj, getFileUrl, children, onDelete }: Props = $props();
	const { id, timestamp, mime, content } = obj;
	const datetime = format(timestamp, 'yyyy-MM-dd HH:mm:ss');
	const isTextContent = mime.startsWith('text/plain') || mime.startsWith('text/x-url');

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

	const copyToClipboard = (text: string, what: string) => {
		navigator.clipboard.writeText(text);
		toastState.message = `${what} copied`;
	};

	const copyText = () => {
		if (mime.startsWith('text/plain')) copyToClipboard((content as TextContent).data, 'Text');
		else if (mime.startsWith('text/x-url')) copyToClipboard((content as LinkContent).url, 'URL');
	};

	const copyLink = () => {
		const url = new URL(window.location.toString());
		url.pathname = getFileUrl(obj, content);
		copyToClipboard(url.toString(), 'Object URL');
	};

	const menuList: Menu[] = [
		{
			label: 'Copy Text',
			icon: faClipboard,
			onClick: copyText,
			hidden: !isTextContent || !mime.startsWith('text/plain')
		},
		{
			label: 'Copy URL',
			icon: faLink,
			onClick: copyText,
			hidden: !isTextContent || !mime.startsWith('text/x-url')
		},
		{
			label: 'Copy Object URL',
			icon: faLink,
			onClick: copyLink,
			hidden: isTextContent
		},
		{
			label: 'Delete Object',
			icon: faTrash,
			onClick: () => onDelete && onDelete(id),
			color: 'red'
		}
	];

	onMount(() => {
		updateElapsed();
		const interval = setInterval(updateElapsed, 5000);
		return () => clearInterval(interval);
	});
</script>

<div class="border-b">
	{@render children?.()}
	<div class="text-sub mb-2 flex items-center justify-start pr-2 pl-4 text-sm">
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
