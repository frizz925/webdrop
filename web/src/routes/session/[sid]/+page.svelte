<script lang="ts">
	import {
		faCopy,
		faEllipsisV,
		faEye,
		faEyeSlash,
		faLink,
		faLock,
		faQrcode,
		faShare,
		faSignOut,
		faTrash,
		faX
	} from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';

	import { page } from '$app/state';
	import * as models from '$lib/models';
	import type { NotificationEvent, NotificationHandlers } from '$lib/notification';
	import { base64URL, sluggify, unbase64URL } from '$lib/utils';

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
	import { copyToClipboard } from '$lib/components/utils';
	import {
		authorizedRequest,
		authorizedURL,
		createAuthKey,
		createMasterKey,
		decodeKDFParams,
		encodeBuffer,
		importMasterKey,
		maybeDecryptObject,
		setCryptoConfig
	} from '$lib/crypto';

	import { getObjects } from '$lib/api/session';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';
	import type { PageData } from './$types';

	const { sid } = page.params;
	const { data }: { data: PageData } = $props();
	const slug = sluggify(sid);
	const session = $state(data.session);
	const encrypted = !!session.crypto;

	let sessionReady = $state(false);
	let sidShown = $state(false);
	let qrcodeShown = $state(false);
	let dropdownShown = $state(false);

	let confirmSessionDelete = $state(false);
	let confirmObjectDelete = $state(false);

	let objects: models.FileObject[] = $state([]);
	let objectIDs = new Set();

	const objectURL = (oid: models.ObjectID) => `/api/session/${sid}/objects/${oid}`;

	const getURL = () => new URL(window.location.toString());
	const getSignedURL = () => {
		const url = getURL();
		const key = window.localStorage.getItem(sid);
		if (key) url.searchParams.set('key', base64URL(key));
		return url;
	};
	const getSignedLink = () => getSignedURL().toString();

	const shareLink = () => {
		navigator.share({
			title: 'WebDrop - Easily share files over the web',
			url: getSignedLink()
		});
	};

	const copyLink = () => copyToClipboard(getSignedLink(), 'Session URL');
	const copySlug = () => copyToClipboard(slug, 'Session ID');

	const showQrcode = () => (qrcodeShown = true);
	const returnToTop = () => window.scrollTo(0, 0);

	const addObject = (obj: models.FileObject) => {
		if (objectIDs.has(obj.id)) return;
		objectIDs.add(obj.id);
		objects.unshift(obj);
	};

	const deleteObject = (oid: models.ObjectID) => {
		if (!objectIDs.has(oid)) return;
		objectIDs.delete(oid);
		const obj = objects.find((obj) => obj.id === oid);
		if (obj) objects.splice(objects.indexOf(obj), 1);
	};

	const onUpload = async (obj: models.FileObject) => {
		addObject(await maybeDecryptObject(obj));
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
		const res = await fetch(`/api/session/${sid}`, authorizedRequest({ method: 'DELETE' }));
		if (!res.ok) {
			toastState.message = '';
			return;
		}
		window.localStorage.removeItem(sid);
		exitSession();
	};

	let selectedObjectID: models.ObjectID;
	const askObjectDelete = (oid: models.ObjectID) => {
		selectedObjectID = oid;
		confirmObjectDelete = true;
	};
	const doObjectDelete = async () => {
		const oid = selectedObjectID;
		await fetch(objectURL(oid), authorizedRequest({ method: 'DELETE' }));
		deleteObject(oid);
		confirmObjectDelete = false;
		toastState.message = 'Object deleted';
	};

	const notificationHandlers: NotificationHandlers = {
		'object.created': async (evt: NotificationEvent) => {
			const oid = evt.data as models.ObjectID;
			const res = await fetch(objectURL(oid), authorizedRequest());
			const obj = (await res.json()) as models.FileObject;
			addObject(await maybeDecryptObject(obj));
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

		ws = new WebSocket(authorizedURL(url));
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

	const setupSessionCrypto = async (config: models.SessionCrypto) => {
		const password = window.localStorage.getItem(sid);
		if (!password)
			throw new Error('Session is encrypted but password was not found in the LocalStorage');

		const kdfParams = decodeKDFParams(config.kdfParams);
		const { masterKey: masterKeyRaw } = await createMasterKey(password, kdfParams);
		const masterKey = await importMasterKey(masterKeyRaw);
		const authKeyRaw = await createAuthKey(masterKeyRaw);
		const authKey = encodeBuffer(authKeyRaw);

		setCryptoConfig({ masterKey, authKey, authKeyURL: base64URL(authKey) });
	};

	const loadObjects = async () => {
		objects = await getObjects(sid);
		objectIDs = new Set(objects.map((o) => o.id));
		await Promise.all(
			objects.map(async (obj, idx) => (objects[idx] = await maybeDecryptObject(obj)))
		);
	};

	const sessionMenuList: () => Menu[] = () => [
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
			icon: faCopy,
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
			color: 'red',
			hidden: !sessionReady
		}
	];

	onMount(async () => {
		const url = getURL();
		const urlKey = url.searchParams.get('key');
		if (urlKey) {
			if (!window.localStorage.getItem(sid)) window.localStorage.setItem(sid, unbase64URL(urlKey));
			url.searchParams.delete('key');
			window.location.replace(url);
		}

		if (session.crypto) await setupSessionCrypto(session.crypto);
		await loadObjects();
		connectWS();
		sessionReady = true;
	});
</script>

<div
	class="fixed left-0 top-0 z-10 flex h-12 w-full items-center justify-center border-b bg-white px-4 dark:bg-slate-800"
>
	<button class="cursor-pointer text-xl font-bold" onclick={returnToTop}>WebDrop</button>
</div>
<div class="mt-12 bg-white dark:bg-slate-800">
	<div class="flex items-center justify-start border-b py-1 pl-4 pr-2">
		<div class="flex grow items-center justify-start">
			<div class="mr-2 text-xs" class:hidden={!encrypted}>
				<FontAwesomeIcon icon={faLock} />
			</div>
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
				<IconButton icon={faCopy} size="xs" onClick={copySlug} />
				<DropdownMenu bind:shown={dropdownShown} menuList={sessionMenuList()}>
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
	<div class="border-b p-4" class:hidden={!sessionReady}>
		<Form {sid} onSubmit={onUpload} />
	</div>
	<div>
		{#each objects as obj (obj.id)}
			{#if obj.content.kind === 'text'}
				<TextContent
					object={obj}
					content={obj.content as models.TextContent}
					onDelete={askObjectDelete}
				/>
			{:else if obj.content.kind === 'link'}
				<LinkContent
					{sid}
					object={obj}
					content={obj.content as models.LinkContent}
					onDelete={askObjectDelete}
				/>
			{:else if obj.content.kind === 'file'}
				{#if obj.mime.startsWith('image/')}
					<ImageContent
						{sid}
						object={obj}
						content={obj.content as models.FileContent}
						onDelete={askObjectDelete}
					/>
				{:else if obj.mime.startsWith('video/')}
					<VideoContent
						{sid}
						object={obj}
						content={obj.content as models.FileContent}
						onDelete={askObjectDelete}
					/>
				{:else if obj.mime.startsWith('audio/')}
					<AudioContent
						{sid}
						object={obj}
						content={obj.content as models.FileContent}
						onDelete={askObjectDelete}
					/>
				{:else}
					<LinkContent
						{sid}
						object={obj}
						content={obj.content as models.FileContent}
						onDelete={askObjectDelete}
						download
					/>
				{/if}
			{/if}
		{/each}
	</div>
</div>
<QRCodeModal bind:shown={qrcodeShown} text={page.url.toString()} />
<ConfirmationModal bind:shown={confirmSessionDelete}>
	<div class="text-xl font-bold">Session termination</div>
	<div class="mt-4">
		<div>Do you want to terminate the session?</div>
		<div class="font-semibold italic text-red-400">Your uploaded files will be deleted!</div>
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
		<div class="font-semibold italic text-red-400">Deleted objects can't be recovered!</div>
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
