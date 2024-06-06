<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import IAnalysisResult from '../lib/project/analyze/IAnalysisResult.svelte';
	import { ArenaTree, Node, type AnalysisResult, type DirectoryImage } from '../lib/types';
	import { Accordion, AccordionItem, ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import IconExclamationTriangle from '$lib/icons/IconExclamationTriangle.svelte';

	import IProjectArenaTree from '$lib/project/structure/IProjectArenaTree.svelte';
	import IProjectViewport from '$lib/project/IProjectViewport.svelte';
	import CarbonVolumeBlockStorage from '$lib/icons/CarbonVolumeBlockStorage.svelte';
	import CarbonScanAlt from '$lib/icons/CarbonScanAlt.svelte';
	import { onMount } from 'svelte';
	import CarbonRepoSourceCode from '$lib/icons/CarbonRepoSourceCode.svelte';
	import CarbonFolderOpen from '$lib/icons/CarbonFolderOpen.svelte';
	import Frame from '$lib/components/Frame.svelte';
	import IConfigurationViewport from '$lib/configuration/IConfigurationViewport.svelte';

	let active_viewport: string = 'project';
	const configuration_viewport_name = 'configuration';
	const project_viewport_name = 'project';

	let ini_project_viewport = false;
	let ini_configuration_viewport = false;

	

	async function show_project() {
		if (ini_project_viewport == false) {
			ini_project_viewport = true;
			console.log('ini_project_viewport: ' + ini_project_viewport);
		}
		active_viewport = project_viewport_name;
		console.log('active_viewport: ' + active_viewport);
	}

	async function show_configuration() {
		if (ini_configuration_viewport == false) {
			ini_configuration_viewport = true;
			console.log('ini_configuration_viewport: ' + ini_configuration_viewport);
		}
		active_viewport = configuration_viewport_name;
		console.log('active_viewport: ' + active_viewport);
	}

	async function reveal_workspace_folder(){
		await invoke('c_reveal_workspace_folder');
	}
</script>

<div class="window">
	<div id="active-bar" class="active-bar variant-soft-surface basis-14">
		<button type="button" class=" btn" on:click={show_project}>
			<CarbonRepoSourceCode class="size-6" />
		</button>
		<button type="button" class=" btn" on:click={show_configuration}>
			<CarbonVolumeBlockStorage class="size-6" />
		</button>
		<button type="button" class=" btn" on:click={reveal_workspace_folder}>
			<CarbonFolderOpen class="size-6" />
		</button>
	
		<!-- <button type="button" class="variant-filled btn" on:click={analyze} id="button">Analyze</button>
		<button type="button" class="variant-filled btn" on:click={analyze_tree} id="button"
			>Analyze tree</button
		>
		
		<button type="button" class="variant-filled btn" on:click={show_configuration} id="button"
			>Show images</button
		> -->
	</div>
	<div id="workspace" class="workspace bg-surface-100">
		<Frame display={active_viewport === configuration_viewport_name ? null : 'none'}>
			{#if ini_configuration_viewport}
				<IConfigurationViewport />
			{/if}
		</Frame>

		<Frame display={active_viewport === project_viewport_name ? null : 'none'}>
			{#if ini_project_viewport}
				<IProjectViewport />
			{/if}
		</Frame>
		<!-- <div id="container">
			<button type="button" class="variant-filled btn" on:click={analyze} id="button"
				>Analyze</button
			>
			<button type="button" class="variant-filled btn" on:click={analyze_tree} id="button"
				>Analyze tree</button
			>
			<button type="button" class="variant-filled btn" on:click={take_directory_image} id="button"
				>Take image</button
			>
			<button type="button" class="variant-filled btn" on:click={show_configuration} id="button"
				>Show images</button
			>
		</div> -->
	</div>
</div>

<style>
	.window {
		display: flex;
		flex-flow: row;
		height: 100vh;
		width: 100vw;
	}

	.active-bar {
		display: flex;
		flex: none;
		flex-flow: column;
	}

	.workspace {
		display: flex;
		flex-flow: column;
		width: 100%;
		height: 100%;
	}
</style>
