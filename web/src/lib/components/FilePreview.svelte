<script lang="ts">
	import {
		faFile,
		faFileArchive,
		faFileAudio,
		faFilePdf,
		faFileText,
		faFilm,
		faImage
	} from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	interface Props {
		file?: File;
		image?: string;
		name: string;
		type: string;
	}

	const { file, image, name, type }: Props = $props();
	let imageSrc = $state(image);

	if (!image && file && file.type.startsWith('image/')) {
		const reader = new FileReader();
		reader.onload = () => {
			imageSrc = reader.result as string;
		};
		reader.readAsDataURL(file);
	}

	const getIcon = () => {
		if (type.startsWith('image/')) return faImage;
		else if (type.startsWith('video/')) return faFilm;
		else if (type.startsWith('audio/')) return faFileAudio;
		else if (type.startsWith('text/')) return faFileText;
		else if (type.startsWith('application/zip')) return faFileArchive;
		else if (type.startsWith('application/pdf')) return faFilePdf;
		else return faFile;
	};
</script>

<div class="p-1">
	<div class="w-36 rounded-md border p-2 shadow-sm">
		{#if imageSrc}
			<img src={imageSrc} alt={name} class="m-auto block h-36 object-contain" />
		{:else}
			<div class="m-auto flex h-36 items-center justify-center text-4xl">
				<FontAwesomeIcon icon={getIcon()} />
			</div>
		{/if}
		<div class="mt-2 overflow-hidden text-center text-xs text-ellipsis whitespace-nowrap">
			{name}
		</div>
	</div>
</div>
