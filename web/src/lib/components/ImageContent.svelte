<script lang="ts">
	import Content, { type PartialProps } from './Content.svelte';

	interface Props extends PartialProps {
		src: string;
		name?: string;
	}

	const { sid, object, src, name, onDelete }: Props = $props();
</script>

<Content {sid} {object} {onDelete}>
	<img {src} alt={name} class="w-full" />
</Content>
