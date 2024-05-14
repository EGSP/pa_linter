<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import IGreet from '../lib/IGreet.svelte';
	import IAnalysisResult from '../lib/IAnalysisResult.svelte';
	import type { AnalysisResult } from '../lib/types';

	let analysis_results = new Array<AnalysisResult>();

	async function analyze() {
		analysis_results = await invoke<AnalysisResult[]>('analyze_folder', {
			folderPath: 'c:/Workroot/softdev/pa_linter_test'
		});

		console.log(analysis_results);
	}
</script>

<h1>Welcome to SvelteKit</h1>
<div id="container">
	<button on:click={analyze} id="button">Analyze</button>
</div>
<div id="results">
	{#if analysis_results.length > 0}
		{#each analysis_results as result}
			<IAnalysisResult 
			file_path="{result.file_path}," 
			tips={result.tips} />
			<hr>
		{/each}
	{:else}
		<p>No results</p>
	{/if}
</div>
<IGreet />
