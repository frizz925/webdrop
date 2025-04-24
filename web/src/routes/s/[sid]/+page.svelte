<script lang="ts">
	import { page } from '$app/state';
	import Form from '$lib/components/Form.svelte';
	import IconButton from '$lib/components/IconButton.svelte';
	import ImageContent from '$lib/components/ImageContent.svelte';
	import LinkContent from '$lib/components/LinkContent.svelte';
	import TextContent from '$lib/components/TextContent.svelte';
	import { sluggify } from '$lib/utils';
	import { faClipboard } from '@fortawesome/free-regular-svg-icons';
	import { faEye, faEyeSlash, faQrcode, faShare } from '@fortawesome/free-solid-svg-icons';

	const slug = sluggify(page.params.sid);
	const tsExample = new Date(2025, 3, 24);
	const textExample = `
	Tung tung tung tung tung tung tung tung tung sahur. Anomali mengerikan yang hanya keluar pada sahur. Konon katanya kalau ada orang yang dipanggil Sahur tiga kali dan tidak nyaut maka makhluk ini datang di rumah kalian. Hiii seremnya. Tung tung ini biasanya bersuara layaknya pukulan kentungan seperti ini tung tung tung tung. Share ke teman kalian yang susah Sahur. 
	`;

	const getLink = () => window.location.toString();

	const copySlug = () => {
		navigator.clipboard.writeText(slug);
	};

	const shareLink = () => {
		navigator.share({
			title: 'WebDrop - Easily share files over the web',
			url: getLink()
		});
	};

	let sidShown = $state(false);
</script>

<div class="bg-color fixed top-0 left-0 flex h-12 w-full items-center justify-center border-b px-4">
	<button class="text-xl font-bold" onclick={() => window.scrollTo(0, 0)}>WebDrop</button>
</div>
<div class="mt-12">
	<div class="flex items-center justify-start border-b px-4 py-1">
		<div class="grow">
			<span class="hidden font-semibold sm:inline">Session ID</span>
			<span class="inline font-semibold sm:hidden">SID</span>
			<span class:hidden={!sidShown}>{slug}</span>
			<span class:hidden={sidShown} class="italic opacity-50"> xxxx-xxxx-xxxx-xxxx </span>
		</div>
		<div class="text-sub flex items-center justify-start">
			<div class:hidden={sidShown}>
				<IconButton icon={faEye} onclick={() => (sidShown = true)} />
			</div>
			<div class:hidden={!sidShown}>
				<IconButton icon={faEyeSlash} onclick={() => (sidShown = false)} />
			</div>
			<IconButton icon={faClipboard} onclick={copySlug} />
		</div>
		<div class="text-sub hidden items-center justify-start sm:flex">
			<IconButton icon={faQrcode} />
			<IconButton icon={faShare} onclick={shareLink} />
		</div>
	</div>
	<div class="border-b p-4">
		<Form />
	</div>
	<div>
		<TextContent content={textExample} timestamp={tsExample} />
		<ImageContent src="https://172.16.0.4/shiki.jpg" name="Shiki" timestamp={tsExample} />
		<LinkContent name="shiki.jpg" url="https://172.16.0.4/shiki.jpg" timestamp={tsExample} />
		<LinkContent
			name="proxy.pac"
			url="https://172.16.0.4/proxy.pac"
			timestamp={tsExample}
			download
		/>
	</div>
</div>
