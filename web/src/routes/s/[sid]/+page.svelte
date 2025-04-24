<script lang="ts">
	import { page } from '$app/state';
	import Form from '$lib/components/Form.svelte';
	import IconButton from '$lib/components/IconButton.svelte';
	import LinkContent from '$lib/components/LinkContent.svelte';
	import QRCodeWindow from '$lib/components/QRCodeWindow.svelte';
	import TextContent from '$lib/components/TextContent.svelte';
	import type * as models from '$lib/models';
	import { sluggify } from '$lib/utils';
	import { faClipboard } from '@fortawesome/free-regular-svg-icons';
	import { faEye, faEyeSlash, faQrcode, faShare } from '@fortawesome/free-solid-svg-icons';
	import type { PageData } from './$types';

	const { sid } = page.params;
	const slug = sluggify(sid);

	const { data }: { data: PageData } = $props();

	let sidShown = $state(false);
	let qrcodeShown = $state(false);
	let session = $state(data.session);

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

	const newObject = (obj: models.FileObject<models.Content>) => {
		session.objects.unshift(obj);
	};
</script>

<div
	class="bg-color fixed top-0 left-0 z-10 flex h-12 w-full items-center justify-center border-b px-4"
>
	<button class="cursor-pointer text-xl font-bold" onclick={returnToTop}>WebDrop</button>
</div>
<div class="mt-12">
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
		<div class="text-sub flex items-center justify-start">
			<div class:hidden={sidShown}>
				<IconButton icon={faEye} onClick={() => (sidShown = true)} />
			</div>
			<div class:hidden={!sidShown}>
				<IconButton icon={faEyeSlash} onClick={() => (sidShown = false)} />
			</div>
			<IconButton icon={faClipboard} onClick={copySlug} />
		</div>
		<div class="text-sub hidden items-center justify-start sm:flex">
			<IconButton icon={faQrcode} onClick={showQrcode} />
			<IconButton icon={faShare} onClick={shareLink} />
		</div>
	</div>
	<div class="border-b p-4">
		<Form {sid} onSubmit={newObject} />
	</div>
	<div>
		{#each session.objects as obj (obj.id)}
			{#if obj.content.kind === 'text'}
				<TextContent content={obj.content as models.TextContent} timestamp={obj.timestamp} />
			{:else if obj.content.kind === 'link'}
				<LinkContent content={obj.content as models.LinkContent} timestamp={obj.timestamp} />
			{/if}
		{/each}
	</div>
</div>
<QRCodeWindow bind:shown={qrcodeShown} />
