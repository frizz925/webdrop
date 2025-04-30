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
		file: File;
		uploading?: boolean;
		progress?: number;
		onRemove?: (file: File) => void;
	}

	const { file, uploading = false, progress = 0.0, onRemove }: Props = $props();
	const { name, type } = file;
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

	const progressStyle = () => {
		const percent = progress * 100;
		return `width: ${percent}%`;
	};
</script>

<div class="relative p-3">
	<button
		class="btn-remove h-8 w-8 rounded-full border bg-white p-4 text-xs shadow-sm dark:bg-gray-800 dark:shadow-gray-900"
		onclick={() => onRemove && onRemove(file)}
		class:flex={!!onRemove}
		class:hidden={!onRemove || uploading}
	>
		<FontAwesomeIcon icon={faX} />
	</button>
	<div class="w-36 overflow-hidden rounded-md border shadow-sm dark:shadow-gray-900" title={name}>
		<div class="px-2 pt-4 pb-2">
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
		<div>
			<div class="h-1 bg-sky-400" style={progressStyle()}></div>
		</div>
	</div>
</div>

<style>
	.btn-remove {
		position: absolute;
		top: 0;
		left: 0;
		justify-content: center;
		align-items: center;
		cursor: pointer;
	}
</style>
