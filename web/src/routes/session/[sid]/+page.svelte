<script lang="ts">
	import { page } from '$app/state';
	import type * as models from '$lib/models';
	import type { NotificationEvent, NotificationHandlers } from '$lib/notification';
	import { sluggify } from '$lib/utils';
	import {
		faClipboard,
		faEllipsisV,
		faEye,
		faEyeSlash,
		faLink,
		faQrcode,
		faShare,
		faSignOut,
		faTrash,
		faX
	} from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';

	import Form from '$lib/components/Form.svelte';
	import IconButton from '$lib/components/buttons/IconButton.svelte';
	import QRCodeModal from '$lib/components/modals/QRCodeModal.svelte';

	import AudioContent from '$lib/components/contents/AudioContent.svelte';
	import ImageContent from '$lib/components/contents/ImageContent.svelte';
	import LinkContent from '$lib/components/contents/LinkContent.svelte';
	import TextContent from '$lib/components/contents/TextContent.svelte';
	import VideoContent from '$lib/components/contents/VideoContent.svelte';

	import DropdownMenu, { type Menu } from '$lib/components/DropdownMenu.svelte';
	import ConfirmationModal from '$lib/components/modals/ConfirmationModal.svelte';

	import Toast from '$lib/components/Toast.svelte';
	import { toastState } from '$lib/components/state.svelte';
	import * as utils from '$lib/utils';
	import type { PageData } from './$types';

	const { sid } = page.params;
	const slug = sluggify(sid);

	const { data }: { data: PageData } = $props();

	let sidShown = $state(false);
	let qrcodeShown = $state(false);
	let dropdownShown = $state(false);

	let confirmSessionDelete = $state(false);
	let confirmObjectDelete = $state(false);

	let session = $state(data.session);

	const objectIDs = new Set(session.objects.map((o) => o.id));

	const getLink = () => window.location.toString();

	const returnToTop = () => {
		window.scrollTo(0, 0);
	};

	const copyToClipboard = (text: string, what: string) => {
		navigator.clipboard.writeText(text);
		toastState.message = `${what} copied`;
	};
	const copyLink = () => copyToClipboard(getLink(), 'Session URL');
	const copySlug = () => copyToClipboard(slug, 'Session ID');

	const shareLink = () => {
		navigator.share({
			title: 'WebDrop - Easily share files over the web',
			url: getLink()
		});
	};

	const getFileUrl = (obj: models.FileObject, content: models.Content) => {
		return utils.getFileUrl(sid, obj, content as models.FileContent);
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

	const onUpload = (obj: models.FileObject) => {
		addObject(obj);
		toastState.message = 'Object uploaded';
	};

	let ws: WebSocket | undefined;
	let exited = false;
	const exitSession = () => {
		if (exited) return;
		exited = true;

		if (ws) ws.close();
		window.location.assign('/');
	};

	const deleteSession = async () => {
		await fetch(`/api/session/${sid}`, { method: 'DELETE' });
		exitSession();
	};

	const notificationHandlers: NotificationHandlers = {
		'object.created': async (evt: NotificationEvent) => {
			const oid = evt.data as models.ObjectID;
			const res = await fetch(`/api/session/${sid}/${oid}`);
			const obj = (await res.json()) as models.FileObject;
			addObject(obj);
		},
		'object.deleted': (evt: NotificationEvent) => {
			deleteObject(evt.data as models.ObjectID);
		},
		'session.deleted': exitSession
	};

	const connectWS = () => {
		const url = new URL(window.location.href);
		url.protocol = url.protocol.replace('http', 'ws');
		url.pathname = `/ws/${sid}`;

		ws = new WebSocket(url);
		ws.onopen = () => {
			console.log('WebSocket connected');
		};
		ws.onmessage = ({ data }) => {
			if (typeof data !== 'string') return;
			const evt = JSON.parse(data as string) as NotificationEvent;
			const handler = notificationHandlers[evt.name];
			if (handler) handler(evt);
		};
		ws.onerror = (e) => {
			console.error('WebSocket error', e);
			console.log('Reconnecting WebSocket in 5 seconds...');
			setTimeout(connectWS, 5000);
		};
		ws.onclose = () => {
			console.log('WebSocket disconnected');
			if (exited) return;

			console.log('Reconnecting WebSocket in 5 seconds...');
			setTimeout(connectWS, 5000);
		};
	};

	let selectedObjectID: models.ObjectID;
	const askObjectDelete = (oid: models.ObjectID) => {
		selectedObjectID = oid;
		confirmObjectDelete = true;
	};
	const doObjectDelete = async () => {
		const oid = selectedObjectID;
		await fetch(`/api/session/${sid}/${oid}`, { method: 'DELETE' });
		deleteObject(oid);
		confirmObjectDelete = false;
		toastState.message = 'Object deleted';
	};

	const sessionMenuList: Menu[] = [
		{
			label: 'Show QR Code',
			icon: faQrcode,
			onClick: showQrcode
		},
		{
			label: 'Share Session',
			icon: faShare,
			onClick: shareLink
		},
		{
			label: 'Copy Session URL',
			icon: faLink,
			onClick: copyLink
		},
		{
			label: 'Copy Session ID',
			icon: faClipboard,
			onClick: copySlug
		},
		{
			label: 'Exit Session',
			icon: faSignOut,
			onClick: exitSession
		},
		{
			label: 'Terminate Session',
			icon: faX,
			onClick: () => (confirmSessionDelete = true),
			color: 'red'
		}
	];

	onMount(connectWS);
</script>

<div
	class="fixed top-0 left-0 z-10 flex h-12 w-full items-center justify-center border-b bg-white px-4 dark:bg-slate-800"
>
	<button class="cursor-pointer text-xl font-bold" onclick={returnToTop}>WebDrop</button>
</div>
<div class="mt-12 bg-white dark:bg-slate-800">
	<div class="flex items-center justify-start border-b py-1 pr-2 pl-4">
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
		<div class="flex items-center justify-start">
			<div class="text-sub" class:hidden={sidShown}>
				<IconButton icon={faEye} size="xs" onClick={() => (sidShown = true)} />
			</div>
			<div class="text-sub" class:hidden={!sidShown}>
				<IconButton icon={faEyeSlash} size="xs" onClick={() => (sidShown = false)} />
			</div>
			<div class="text-sub hidden items-center justify-start sm:flex">
				<IconButton icon={faQrcode} size="xs" onClick={showQrcode} />
				<IconButton icon={faShare} size="xs" onClick={shareLink} />
			</div>
			<div class="text-sub flex items-center justify-start">
				<IconButton icon={faClipboard} size="xs" onClick={copySlug} />
				<DropdownMenu bind:shown={dropdownShown} menuList={sessionMenuList}>
					<IconButton
						icon={faEllipsisV}
						size="xs"
						onClick={() => (dropdownShown = !dropdownShown)}
					/>
				</DropdownMenu>
			</div>
		</div>
		<div class="hidden">
			<IconButton
				icon={faTrash}
				size="xs"
				hoverBgColor="red"
				onClick={() => (confirmSessionDelete = true)}
			/>
		</div>
	</div>
	<div class="border-b p-4">
		<Form {sid} onSubmit={onUpload} />
	</div>
	<div>
		{#each session.objects as obj (obj.id)}
			{#if obj.content.kind === 'text'}
				<TextContent
					object={obj}
					content={obj.content as models.TextContent}
					{getFileUrl}
					onDelete={askObjectDelete}
				/>
			{:else if obj.content.kind === 'link'}
				<LinkContent
					object={obj}
					content={obj.content as models.LinkContent}
					{getFileUrl}
					onDelete={askObjectDelete}
				/>
			{:else if obj.content.kind === 'file'}
				{#if obj.mime.startsWith('image/')}
					<ImageContent {sid} object={obj} content={obj.content as models.FileContent} />
				{:else if obj.mime.startsWith('video/')}
					<VideoContent {sid} object={obj} content={obj.content as models.FileContent} />
				{:else if obj.mime.startsWith('audio/')}
					<AudioContent {sid} object={obj} content={obj.content as models.FileContent} />
				{/if}
				<LinkContent
					object={obj}
					content={obj.content as models.FileContent}
					{getFileUrl}
					onDelete={askObjectDelete}
					download
				/>
			{/if}
		{/each}
	</div>
</div>
<QRCodeModal bind:shown={qrcodeShown} text={page.url.toString()} />
<ConfirmationModal bind:shown={confirmSessionDelete}>
	<div class="text-xl font-bold">Session termination</div>
	<div class="mt-4">
		<div>Do you want to terminate the session?</div>
		<div class="font-semibold text-red-400 italic">Your uploaded files will be deleted!</div>
	</div>
	<div class="mt-8 text-right">
		<button
			class="cursor-pointer rounded-md p-2 transition-colors hover:bg-gray-500/20"
			onclick={() => (confirmSessionDelete = false)}>Cancel</button
		>
		<button
			class="cursor-pointer rounded-md bg-red-400 p-2 text-gray-50 shadow shadow-transparent transition-shadow duration-150 hover:shadow-red-400"
			onclick={deleteSession}
		>
			Terminate
		</button>
	</div>
</ConfirmationModal>
<ConfirmationModal bind:shown={confirmObjectDelete}>
	<div class="text-xl font-bold">Delete object</div>
	<div class="mt-4">
		<div>Are you sure you want to delete this object?</div>
		<div class="font-semibold text-red-400 italic">Deleted objects can't be recovered!</div>
	</div>
	<div class="mt-8 text-right">
		<button
			class="cursor-pointer rounded-md p-2 transition-colors hover:bg-gray-500/20"
			onclick={() => (confirmObjectDelete = false)}>Cancel</button
		>
		<button
			class="cursor-pointer rounded-md bg-red-400 p-2 text-gray-50 shadow shadow-transparent transition-shadow duration-150 hover:shadow-red-400"
			onclick={doObjectDelete}
		>
			Delete
		</button>
	</div>
</ConfirmationModal>
<Toast />
