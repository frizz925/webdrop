<script lang="ts">
	import {
		faFile,
		faFileArchive,
		faFileAudio,
		faFilePdf,
		faFileText,
		faFilm,
		faImage,
		faX
	} from '@fortawesome/free-solid-svg-icons';
	import { FontAwesomeIcon } from '@fortawesome/svelte-fontawesome';

	interface Props {
		file?: File;
		name: string;
		type: string;
		uploading?: boolean;
		onRemove?: (file?: File) => void;
	}

	const { file, name, type, uploading = false, onRemove }: Props = $props();
	let imageSrc: string | undefined = $state();

	if (file && file.type.startsWith('image/')) {
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

<div class="p-3">
	<div
		class="relative w-36 rounded-md border px-2 pt-4 pb-2 shadow-sm dark:shadow-gray-900"
		title={name}
	>
		<button
			class="btn-remove -top-4 -left-4 h-8 w-8 rounded-full border bg-white p-4 text-xs shadow-sm dark:bg-gray-800 dark:shadow-gray-900"
			onclick={() => onRemove && onRemove(file)}
			class:flex={!!onRemove}
			class:hidden={!onRemove || uploading}
		>
			<FontAwesomeIcon icon={faX} />
		</button>
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

<style>
	.btn-remove {
		position: absolute;
		justify-content: center;
		align-items: center;
		cursor: pointer;
	}
</style>
