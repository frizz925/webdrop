<script lang="ts">
	import { FormState } from '$lib/form';
	import * as models from '$lib/models';
	import { jsonRequest } from '$lib/utils';
	import {
		faFile,
		faFilm,
		faImage,
		faLink,
		faMicrophone,
		faPencil,
		faPlus
	} from '@fortawesome/free-solid-svg-icons';

	import { onMount } from 'svelte';
	import FormButtons from './buttons/FormButtons.svelte';
	import IconButton from './buttons/IconButton.svelte';
	import FilePreview from './FilePreview.svelte';

	interface Props {
		sid: string;
		onSubmit: (obj: models.FileObject) => void;
	}

	interface Upload {
		id: symbol;
		file: File;
		progress: number;
		finished: boolean;
		xhr?: XMLHttpRequest;
	}

	interface State {
		form: FormState;
		text: string;
		url: {
			value: string;
			title?: string;
		};
		uploads: Upload[];
		message: string;
		uploading: boolean;
	}

	const initialState = () =>
		({
			form: FormState.None,
			text: '',
			url: { value: '' },
			uploads: [],
			message: '',
			uploading: false
		}) as State;

	let { sid, onSubmit }: Props = $props();
	let state: State = $state(initialState());
	let urlInput: HTMLInputElement;
	let fileInput: HTMLInputElement;

	const acceptMap = {
		[FormState.None]: '',
		[FormState.Text]: 'text/*',
		[FormState.Link]: 'text/x-uri',
		[FormState.Image]: 'image/*',
		[FormState.Video]: 'video/*',
		[FormState.Audio]: 'audio/*',
		[FormState.File]: '*'
	};

	const resetState = () => {
		urlInput.value = '';
		fileInput.value = '';
		state = initialState();
	};

	const changeState = (newState: FormState) => () => (state.form = newState);

	const fileListToUploads = (fileList: FileList) => {
		const uploads = [];
		for (const file of fileList)
			uploads.push({ id: Symbol(), file, progress: 0.0, finished: false } as Upload);
		return uploads;
	};

	const updateFiles = (files: FileList, nextState?: FormState) => {
		if (files.length <= 0) return;
		const uploads = fileListToUploads(files);
		const nextUploads = nextState ? [...state.uploads, ...uploads] : uploads;
		if (nextState) state = { ...state, form: nextState, uploads: nextUploads };
		else state.uploads = nextUploads;
	};

	const selectFiles = (nextState?: FormState) => () => {
		nextState = nextState || state.form;
		fileInput.value = '';
		fileInput.accept = acceptMap[nextState];
		fileInput.onchange = (evt) => {
			const el = evt.target as HTMLInputElement;
			if (!el.files) return;
			updateFiles(el.files, nextState);
		};
		fileInput.click();
	};

	const stateIsFile = (state: FormState) => {
		return (
			state === FormState.Image ||
			state === FormState.Video ||
			state === FormState.Audio ||
			state === FormState.File
		);
	};

	const textInputValid = (value: string) => {
		return !state.uploading && value.trim().length > 0;
	};

	const updateURL = (evt: Event) => {
		const el = evt.target as HTMLInputElement;
		const value = el.value.trim();
		try {
			new URL(el.value.trim());
			state.url.value = value;
		} catch {
			state.url.value = '';
		}
	};

	const processClipboard = (evt: ClipboardEvent) => {
		if (state.text.trim().length > 0) return;
		const files = evt.clipboardData?.files;
		if (!files || files.length <= 0) return;
		evt.preventDefault();
		updateFiles(files, FormState.File);
	};

	const removeFile = (file: File) => {
		const uploads = state.uploads.filter(({ file: other }) => other !== file);
		const form = uploads.length <= 0 ? FormState.None : state.form;
		state = { ...state, uploads, form };
	};

	const submit = async <C extends models.Content>(mime: string, content: C) => {
		const upload: models.Upload<C> = { mime, content };
		state.uploading = true;
		const res = await fetch(`/api/session/${sid}`, jsonRequest('POST', upload));
		if (res.status >= 400) {
			state = { ...state, uploading: false, message: 'Failed to send' };
			throw res;
		}
		const dto: models.FileObjectDto<C> = await res.json();
		resetState();
		onSubmit(models.objectFromDto(dto));
	};

	const submitText = async () =>
		submit<models.TextContent>('text/plain', {
			kind: 'text',
			data: state.text
		});

	const submitURL = async () =>
		submit<models.LinkContent>('text/x-url', {
			kind: 'link',
			url: state.url.value,
			title: state.url.title
		});

	const uploadFile = async (upload: Upload) => {
		const { file } = upload;
		const data = new FormData();
		data.append('file', file, file.name);

		const xhr = new XMLHttpRequest();
		upload.xhr = xhr;

		const promise = new Promise<models.FileObjectDto>((resolve, reject) => {
			xhr.onload = () => {
				upload.progress = 1.0;
				upload.finished = true;
				resolve(JSON.parse(xhr.responseText) as models.FileObjectDto);
			};
			xhr.onerror = () => {
				reject(xhr);
			};
		});
		xhr.upload.onprogress = (evt) => {
			upload.progress = (evt.loaded / evt.total) * 0.85;
		};
		xhr.onabort = () => {
			upload.progress = 0.0;
		};
		xhr.open('POST', `/objects/${sid}`);
		xhr.send(data);

		try {
			const dto = await promise;
			onSubmit(models.objectFromDto(dto));
		} catch (err) {
			state = { ...state, uploading: false, message: 'Failed to upload' };
			throw err;
		}
	};

	const uploadFiles = async () => {
		const uploads = state.uploads;
		if (uploads.length <= 0) return;
		state.uploading = true;
		for (const upload of uploads) {
			if (!upload.finished) await uploadFile(upload);
		}
		resetState();
	};

	const cancelUpload = () => {
		for (const upload of state.uploads) {
			upload.xhr?.abort();
			if (!upload.finished) upload.progress = 0.0;
		}
		state.uploading = false;
	};

	const globalClipboard = (evt: ClipboardEvent) => {
		const data = evt.clipboardData;
		if (!data) return;
		if (data.files.length > 0) return processClipboard(evt);
		if (state.form !== FormState.None) return;
		const text = data.getData('text');
		try {
			new URL(text);
			urlInput.value = text;
			state = { ...state, form: FormState.Link, url: { value: text } };
		} catch {
			state = { ...state, form: FormState.Text, text };
		}
	};

	onMount(() => {
		document.addEventListener('paste', globalClipboard);
		return () => document.removeEventListener('paste', globalClipboard);
	});
</script>

<input type="file" class="hidden" multiple bind:this={fileInput} />
<div
	class="text-accent flex items-center justify-center text-lg"
	class:hidden={state.form !== FormState.None}
>
	<div class="text-accent flex items-center justify-start">
		<IconButton hoverBgColor="sky" icon={faPencil} onClick={changeState(FormState.Text)} />
		<IconButton hoverBgColor="sky" icon={faLink} onClick={changeState(FormState.Link)} />
		<IconButton hoverBgColor="sky" icon={faImage} onClick={selectFiles(FormState.Image)} />
		<IconButton hoverBgColor="sky" icon={faFilm} onClick={selectFiles(FormState.Video)} />
		<IconButton hoverBgColor="sky" icon={faMicrophone} onClick={selectFiles(FormState.Audio)} />
		<IconButton hoverBgColor="sky" icon={faFile} onClick={selectFiles(FormState.File)} />
	</div>
</div>
<div class:hidden={state.form !== FormState.Text}>
	<div class="relative">
		<div class="textarea mb-4" contenteditable="plaintext-only" bind:innerText={state.text}></div>
		<div
			class="pointer-events-none absolute top-0 left-0 text-gray-500"
			class:hidden={textInputValid(state.text)}
		>
			Enter your message here
		</div>
	</div>
	<FormButtons
		message={state.message}
		disabled={!textInputValid(state.text)}
		onCancel={resetState}
		onSubmit={submitText}
	/>
</div>
<div class="flex flex-col" class:hidden={state.form !== FormState.Link}>
	<input type="text" placeholder="URL" class="mb-4" oninput={updateURL} bind:this={urlInput} />
	<input type="text" placeholder="Title (optional)" class="mb-4" bind:value={state.url.title} />
	<FormButtons
		message={state.message}
		disabled={!textInputValid(state.url.value)}
		onCancel={resetState}
		onSubmit={submitURL}
	/>
</div>
<div class="flex flex-col" class:hidden={!stateIsFile(state.form)}>
	<div class="mb-4 flex flex-wrap items-center justify-center">
		{#each state.uploads as upload (upload.id)}
			<FilePreview
				file={upload.file}
				progress={upload.progress}
				uploading={state.uploading}
				onRemove={removeFile}
			/>
		{/each}
		<div class:hidden={state.uploading}>
			<IconButton icon={faPlus} onClick={selectFiles()} />
		</div>
	</div>
	<FormButtons
		disabled={state.uploading}
		onCancel={state.uploading ? cancelUpload : resetState}
		onSubmit={uploadFiles}
	/>
</div>

<style>
	.textarea {
		background: transparent;
		width: 100%;
		border: none;
		outline: none;
		padding: 0;
		box-shadow: none;
		overflow: auto;
		resize: none;
	}
</style>
