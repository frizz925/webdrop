<script lang="ts">
	import { page } from '$app/state';
	import Form from '$lib/components/Form.svelte';
	import IconButton from '$lib/components/IconButton.svelte';
	import ImageContent from '$lib/components/ImageContent.svelte';
	import TextContent from '$lib/components/TextContent.svelte';
	import { faClipboard } from '@fortawesome/free-regular-svg-icons';
	import { faQrcode, faShare } from '@fortawesome/free-solid-svg-icons';

	const SLUG_PART_LENGTH = 4;

	function sluggify(s: string): string {
		const parts = [];

		let idx = 0;
		while (idx < s.length - SLUG_PART_LENGTH) {
			const [start, end] = [idx, idx + SLUG_PART_LENGTH];
			parts.push(s.substring(start, end));
			idx = end;
		}
		parts.push(s.substring(idx));

		return parts.join('-');
	}

	const slug = sluggify(page.params.sid);
	const tsExample = new Date(2025, 3, 24);
	const textExample = `
	Tung tung tung tung tung tung tung tung tung sahur. Anomali mengerikan yang hanya keluar pada sahur. Konon katanya kalau ada orang yang dipanggil Sahur tiga kali dan tidak nyaut maka makhluk ini datang di rumah kalian. Hiii seremnya. Tung tung ini biasanya bersuara layaknya pukulan kentungan seperti ini tung tung tung tung. Share ke teman kalian yang susah Sahur. 
	`;
</script>

<div class="mx-auto max-w-xl md:border-r md:border-l">
	<div class="flex items-center justify-start border-b px-4 py-1">
		<div class="grow">
			<span class="hidden font-semibold md:inline">Session ID</span>
			<span class="inline font-semibold md:hidden">SID</span>
			<span>{slug}</span>
		</div>
		<div class="text-sub hidden items-center justify-start md:flex">
			<IconButton icon={faClipboard} />
			<IconButton icon={faQrcode} />
			<IconButton icon={faShare} />
		</div>
		<div class="text-sub md:hidden">
			<IconButton icon={faShare} />
		</div>
	</div>
	<div class="border-b p-4">
		<Form />
	</div>
	<div>
		<TextContent content={textExample} timestamp={tsExample} />
		<ImageContent src="https://172.16.0.4/shiki.jpg" name="Shiki" timestamp={tsExample} />
	</div>
</div>
