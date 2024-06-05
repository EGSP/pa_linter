<script lang="ts">
	import { ListBox } from '@skeletonlabs/skeleton';
	import type { DirectoryImage } from '$lib/types';
	import IDirectoryImage from './IDirectoryImage.svelte';
	import { Button } from 'carbon-components-svelte';
	import CarbonAddAlt from '$lib/icons/CarbonAddAlt.svelte';
	import Frame from '$lib/components/Frame.svelte';
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api';

	export let show_images = () => {};
	export let directory_images: DirectoryImage[] = [];

	async function add_image() {
		const selected = await open({
			directory: true,
			multiple: false
		});

		if (!selected) {
			return;
		}
		console.log('Selected path: ' + selected);

		await invoke('c_add_repository', {
			folderPath: selected
		});

		await show_images();
	}
</script>

<Frame>
	<div id="buttons">
		<Button
			on:click={add_image}
			kind="secondary"
			size="small"
			icon={CarbonAddAlt}
			iconDescription="Add directory image"
		/>
	</div>
	<ListBox disabled>
		{#if directory_images && directory_images.length > 0}
			{#each directory_images as directory_image}
				<IDirectoryImage
					bind_value={directory_image.name}
					{directory_image}
					directory_image_files_count={directory_image.files.length}
				/>
			{/each}
		{/if}
	</ListBox>
</Frame>
