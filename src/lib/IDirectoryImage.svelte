<script lang="ts">
	import { ListBoxItem } from '@skeletonlabs/skeleton';
	import type { DirectoryImage } from './types';
	import IconDirectoryImage from '$lib/icons/IconDirectoryImage.svelte';

	export let directory_image: DirectoryImage;
	export let directory_image_files_count: number;
	export let bind_value: string;
</script>

<ListBoxItem bind:group={bind_value} value={directory_image.name} name="directory_image_item">
	<svelte:fragment slot="lead"><IconDirectoryImage /></svelte:fragment>
	<span>{directory_image.name}</span>
    <svelte:fragment slot="trail">{directory_image_files_count}</svelte:fragment>
</ListBoxItem>
