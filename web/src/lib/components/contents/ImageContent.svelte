<script lang="ts">
	import { type FileContent } from '$lib/models';
	import { getFileUrl } from '$lib/utils';
	import { faImage } from '@fortawesome/free-solid-svg-icons';
	import type { Menu } from '../DropdownMenu.svelte';
	import LinkContent from './LinkContent.svelte';
	import type { Props } from './MediaContent';
	import { copyToClipboard } from './utils';

	const { sid, object: obj, content, onDelete }: Props = $props();
	const src = getFileUrl(sid, obj, content);
	let img: HTMLImageElement;

	const copyImage = async () => {
		const canvas = document.createElement('canvas');
		canvas.width = img.naturalWidth;
		canvas.height = img.naturalHeight;

		const context = canvas.getContext('2d');
		if (!context) throw new Error("Can't get Canvas 2D context");
		context.drawImage(img, 0, 0);
		canvas.toBlob((blob) => blob && copyToClipboard(blob, 'Image'));
	};

	const copyMenu: Menu = {
		label: 'Copy Image',
		icon: faImage,
		onClick: copyImage
	};
</script>

<a href={src} target="_blank">
	<img {src} alt={content.name} class="w-full" bind:this={img} />
</a>
<LinkContent
	link={src}
	object={obj}
	content={obj.content as FileContent}
	{copyMenu}
	{onDelete}
	download
/>
