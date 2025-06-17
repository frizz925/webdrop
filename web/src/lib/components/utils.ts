import { toastState } from './state.svelte';

export const copyToClipboard = (content: string | Blob, what: string, mime?: string) => {
	if (content instanceof Blob) {
		console.log(content);
		const type = mime || content.type;
		const item = new ClipboardItem({ [type]: content });
		navigator.clipboard.write([item]);
	} else navigator.clipboard.writeText(content);
	toastState.message = `${what} copied`;
};
