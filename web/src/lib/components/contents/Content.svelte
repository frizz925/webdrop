<script lang="ts">
	import type { FileObject, ObjectID, SessionID } from '$lib/models';
	import { faTrash } from '@fortawesome/free-solid-svg-icons';
	import { format, formatDistanceToNowStrict } from 'date-fns';
	import { onMount } from 'svelte';
	import IconButton from '../buttons/IconButton.svelte';

	export interface PartialProps {
		sid: SessionID;
		object: FileObject;
		children?: any;
		onDelete?: (oid: ObjectID) => void;
	}

	interface Props extends PartialProps {
		children?: any;
	}

	const { sid, object, children, onDelete }: Props = $props();
	const { id, timestamp } = object;
	const datetime = format(timestamp, 'yyyy-MM-dd HH:mm:ss');

	let showTimestamp = $state(false);
	const toggleTimestamp = () => {
		showTimestamp = !showTimestamp;
	};

	let elapsed = $state('Just now');
	const updateElapsed = () => {
		elapsed = formatDistanceToNowStrict(timestamp) + ' ago';
	};

	const deleteObject = async () => {
		await fetch(`/api/session/${sid}/${id}`, { method: 'DELETE' });
		onDelete && onDelete(id);
	};

	onMount(() => {
		updateElapsed();
		const interval = setInterval(updateElapsed, 5000);
		return () => clearInterval(interval);
	});
</script>

<div class="border-b">
	{@render children()}
	<div class="text-sub mb-2 flex items-center justify-start pr-2 pl-4 text-sm">
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="grow">
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
		<div class="block cursor-pointer text-red-400 dark:text-red-800">
			<IconButton icon={faTrash} hoverBgColor="red" size="xs" onClick={deleteObject} />
		</div>
	</div>
</div>
