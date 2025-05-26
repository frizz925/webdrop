<script lang="ts">
	import type { FileObject, ObjectID, SessionID, TextContent } from '$lib/models';
	import { faClipboard, faTrash } from '@fortawesome/free-solid-svg-icons';
	import { format, formatDistanceToNowStrict } from 'date-fns';
	import { onMount, type Snippet } from 'svelte';
	import IconButton from '../buttons/IconButton.svelte';

	export interface PartialProps {
		sid: SessionID;
		object: FileObject;
		children?: Snippet;
		onDelete?: (oid: ObjectID) => void;
	}

	interface Props extends PartialProps {
		children?: Snippet;
	}

	const { sid, object, children, onDelete }: Props = $props();
	const { id, timestamp, mime, content } = object;
	const datetime = format(timestamp, 'yyyy-MM-dd HH:mm:ss');
	const isTextContent = mime.startsWith('text/plain');

	let showTimestamp = $state(false);
	const toggleTimestamp = () => {
		showTimestamp = !showTimestamp;
	};

	let elapsed = $state('Just now');
	const updateElapsed = () => {
		elapsed = formatDistanceToNowStrict(timestamp) + ' ago';
	};

	const copyText = () => {
		if (!isTextContent) return;
		const textContent = content as TextContent;
		navigator.clipboard.writeText(textContent.data);
	};

	const deleteObject = async () => {
		await fetch(`/api/session/${sid}/${id}`, { method: 'DELETE' });
		if (onDelete) onDelete(id);
	};

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
				<span class={!showTimestamp ? 'inline' : 'hidden'}>
					{elapsed}
				</span>
				<span class={showTimestamp ? 'inline' : 'hidden'}>
					{datetime}
				</span>
			</div>
		</div>
		<div class="block cursor-pointer" class:hidden={!isTextContent}>
			<IconButton icon={faClipboard} size="xs" onClick={copyText} />
		</div>
		<div class="block cursor-pointer text-red-400">
			<IconButton icon={faTrash} hoverBgColor="red" size="xs" onClick={deleteObject} />
		</div>
	</div>
</div>
