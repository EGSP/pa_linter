<script lang="ts">
	import { ListBox } from '@skeletonlabs/skeleton';
	import type { DirectoryImage } from '$lib/types';
	import IDirectoryImage from './IDirectoryImage.svelte';
	import { Button } from 'carbon-components-svelte';
	import TdesignRefresh from '$lib/icons/TdesignRefresh.svelte';
	import CarbonScanAlt from '$lib/icons/CarbonScanAlt.svelte';
	import Frame from '$lib/components/Frame.svelte';
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import Label from '$lib/components/Label.svelte';

	let directory_images: DirectoryImage[] = [];
	async function get_directory_images() {
		directory_images = await invoke('c_get_directory_images');
	}

	async function take_directory_image() {
		const selected = await open({
			directory: true,
			multiple: false
		});
		if (!selected) {
			return;
		}

		console.log('Selected path: ' + selected);

		await invoke('c_take_directory_image', {
			folderPath: selected
		});

		await get_directory_images();
	}

	onMount(async () => {
		await get_directory_images();
	});
</script>

<Frame direction="column">
	<Label text = "DIRECTORY IMAGES"/>
	<div id="buttons">
		<Button
			on:click={take_directory_image}
			kind="primary"
			size="small"
			icon={CarbonScanAlt}
			iconDescription="Take directory image"
		/>
		<Button
			on:click={get_directory_images}
			kind="secondary"
			size="small"
			icon={TdesignRefresh}
			iconDescription="Refresh directory images"
		/>
	</div>
	{#if directory_images && directory_images.length > 0}
		<ListBox disabled>
			{#each directory_images as directory_image}
				<IDirectoryImage
					bind_value={directory_image.name}
					{directory_image}
					directory_image_files_count={directory_image.files.length}
				/>
			{/each}
		</ListBox>
	{:else}
		<div>NO DIRECTORY IMAGES</div>
	{/if}
</Frame>
