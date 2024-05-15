<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import IGreet from '../lib/IGreet.svelte';
	import IAnalysisResult from '../lib/IAnalysisResult.svelte';
	import type { AnalysisResult } from '../lib/types';
	import { Accordion, AccordionItem, ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import IDocumentMagnifyingGlass from '$lib/icons/IconDocumentMagnifyingGlass.svelte';
	import IconDocumentMagnifyingGlass from '$lib/icons/IconDocumentMagnifyingGlass.svelte';
	import IconExclamationTriangle from '$lib/icons/IconExclamationTriangle.svelte';

	let analysis_results = new Array<AnalysisResult>();

	async function analyze() {
		analysis_results = await invoke<AnalysisResult[]>('analyze_folder', {
			folderPath: 'c:/Workroot/softdev/pa_linter_test'
		});

		console.log(analysis_results);
	}
	
</script>

<div id="container">
	<button type="button" class="variant-filled btn" on:click={analyze} id="button">Analyze</button>
</div>
<div class="variant-ringed-surface" id="results">
	<Accordion padding="py-2 px-4">
		{#if analysis_results.length > 0}
			{#each analysis_results as result, i}
				<AccordionItem class="variant-ringed-surface">
					<svelte:fragment slot="lead"><IconExclamationTriangle/></svelte:fragment>
					<svelte:fragment slot="summary">{result.file_path}</svelte:fragment>
					<svelte:fragment slot="content">
						<IAnalysisResult file_path="{result.file_path}," tips={result.tips} />
					</svelte:fragment>
				</AccordionItem>
			{/each}
		{:else}
			<p>No results</p>
		{/if}
	</Accordion>
</div>

