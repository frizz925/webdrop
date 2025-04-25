<script lang="ts">
	import type { FileObject, SessionID } from '$lib/models';
	import { format, formatDistanceToNowStrict } from 'date-fns';
	import { onMount } from 'svelte';

	export interface PartialProps {
		sid: SessionID;
		object: FileObject;
		children?: any;
		onDelete?: (obj: FileObject) => void;
	}

	interface Props extends PartialProps {
		children?: any;
	}

	const { sid, object: obj, children, onDelete }: Props = $props();
	const { id, timestamp } = obj;
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
		onDelete && onDelete(obj);
	};

	onMount(() => {
		updateElapsed();
		const interval = setInterval(updateElapsed, 5000);
		return () => clearInterval(interval);
	});
</script>

<div class="border-b">
	{@render children()}
	<div class="text-sub flex items-center justify-start p-4 text-sm">
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
		<button class="block cursor-pointer text-red-500" onclick={deleteObject}>Delete</button>
	</div>
</div>
