<script lang="ts">
	import { page } from '$app/state';
	import { SID_KEY } from '$lib';
	import Form from '$lib/components/Form.svelte';
	import IconButton from '$lib/components/IconButton.svelte';
	import ImageContent from '$lib/components/ImageContent.svelte';
	import LinkContent from '$lib/components/LinkContent.svelte';
	import QRCodeWindow from '$lib/components/QRCodeWindow.svelte';
	import TextContent from '$lib/components/TextContent.svelte';
	import type * as models from '$lib/models';
	import type { NotificationEvent, NotificationHandlers, ObjectEvent } from '$lib/notification';
	import { sluggify } from '$lib/utils';
	import {
		faClipboard,
		faEye,
		faEyeSlash,
		faQrcode,
		faShare,
		faTrash
	} from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import VideoContent from '$lib/components/VideoContent.svelte';
	import AudioContent from '$lib/components/AudioContent.svelte';

	const { sid } = page.params;
	const slug = sluggify(sid);

	const { data }: { data: PageData } = $props();

	let sidShown = $state(false);
	let qrcodeShown = $state(false);
	let session = $state(data.session);

	const objectIDs = new Set(session.objects.map((o) => o.id));

	const getLink = () => window.location.toString();

	const returnToTop = () => {
		window.scrollTo(0, 0);
	};

	const copySlug = () => {
		navigator.clipboard.writeText(slug);
	};

	const shareLink = () => {
		navigator.share({
			title: 'WebDrop - Easily share files over the web',
			url: getLink()
		});
	};

	const showQrcode = () => {
		qrcodeShown = true;
	};

	const addObject = (obj: models.FileObject) => {
		if (objectIDs.has(obj.id)) return;
		objectIDs.add(obj.id);
		session.objects.unshift(obj);
	};

	const deleteObject = (oid: models.ObjectID) => {
		if (!objectIDs.has(oid)) return;
		objectIDs.delete(oid);
		session.objects = session.objects.filter((other) => other.id != oid);
	};

	let ws: WebSocket | undefined;
	const exitSession = () => {
		if (ws) ws.close();
		window.location.assign('/');
	};

	const deleteSession = async () => {
		// TODO: Create confirmation dialog
		await fetch(`/api/session/${sid}`, { method: 'DELETE' });
		localStorage.removeItem(SID_KEY);
		exitSession();
	};

	const notificationHandlers: NotificationHandlers = {
		'object.created': async (evt: NotificationEvent) => {
			const oid = (evt as ObjectEvent).objectID;
			const res = await fetch(`/api/session/${sid}/${oid}`);
			const obj = (await res.json()) as models.FileObject;
			addObject(obj);
		},
		'object.deleted': (evt: NotificationEvent) => {
			deleteObject((evt as ObjectEvent).objectID);
		},
		'session.deleted': exitSession
	};

	const connectWS = () => {
		const url = new URL(window.location.href);
		url.protocol = url.protocol.replace('http', 'ws');
		url.pathname = `/ws/${sid}`;

		ws = new WebSocket(url);
		ws.onopen = () => {
			console.log('WebSocket connected')
		};
		ws.onmessage = ({ data }) => {
			if (typeof data !== 'string') return;
			const evt = JSON.parse(data as string) as NotificationEvent;
			const handler = notificationHandlers[evt.name];
			handler && handler(evt);
		};
		ws.onerror = (e) => {
			console.error('WebSocket error', e);
			connectWS();
		};
		ws.onclose = () => {
			console.log('WebSocket disconnected')
		};
	}

	onMount(connectWS);
</script>

<div
	class="fixed top-0 left-0 z-10 flex h-12 w-full items-center justify-center border-b bg-white px-4 dark:bg-gray-800"
>
	<button class="cursor-pointer text-xl font-bold" onclick={returnToTop}>WebDrop</button>
</div>
<div class="mt-12 bg-white dark:bg-gray-800">
	<div class="flex items-center justify-start border-b px-4 py-1">
		<div class="flex grow items-center justify-start">
			<div class="mr-2">
				<span class="hidden font-semibold sm:inline">Session ID</span>
				<span class="inline font-semibold sm:hidden">SID</span>
			</div>
			<div>
				<span class:hidden={!sidShown}>{slug}</span>
				<span class:hidden={sidShown} class="italic opacity-50">xxxx-xxxx-xxxx-xxxx</span>
			</div>
		</div>
		<div class="text-sub flex items-center justify-start">
			<div class:hidden={sidShown}>
				<IconButton icon={faEye} size="xs" onClick={() => (sidShown = true)} />
			</div>
			<div class:hidden={!sidShown}>
				<IconButton icon={faEyeSlash} size="xs" onClick={() => (sidShown = false)} />
			</div>
			<IconButton icon={faClipboard} size="xs" onClick={copySlug} />
		</div>
		<div class="text-sub hidden items-center justify-start sm:flex">
			<IconButton icon={faQrcode} size="xs" onClick={showQrcode} />
			<IconButton icon={faShare} size="xs" onClick={shareLink} />
		</div>
		<div class="text-red-400 dark:text-red-800">
			<IconButton icon={faTrash} size="xs" hoverBgColor="red" onClick={deleteSession} />
		</div>
	</div>
	<div class="border-b p-4">
		<Form {sid} onSubmit={addObject} />
	</div>
	<div>
		{#each session.objects as obj (obj.id)}
			{#if obj.content.kind === 'text'}
				<TextContent
					{sid}
					object={obj}
					content={obj.content as models.TextContent}
					onDelete={deleteObject}
				/>
			{:else if obj.content.kind === 'link'}
				<LinkContent
					{sid}
					object={obj}
					content={obj.content as models.LinkContent}
					onDelete={deleteObject}
				/>
			{:else if obj.content.kind === 'file'}
				{#if obj.mime.startsWith('image/')}
					<ImageContent
						{sid}
						object={obj}
						content={obj.content as models.FileContent}
						onDelete={deleteObject}
					/>
				{:else if obj.mime.startsWith('video/')}
					<VideoContent
						{sid}
						object={obj}
						content={obj.content as models.FileContent}
						onDelete={deleteObject}
					/>
				{:else if obj.mime.startsWith('audio/')}
					<AudioContent
						{sid}
						object={obj}
						content={obj.content as models.FileContent}
						onDelete={deleteObject}
					/>
				{:else}
					<LinkContent
						{sid}
						object={obj}
						content={obj.content as models.FileContent}
						onDelete={deleteObject}
						download
					/>
				{/if}
			{/if}
		{/each}
	</div>
</div>
<QRCodeWindow bind:shown={qrcodeShown} />
