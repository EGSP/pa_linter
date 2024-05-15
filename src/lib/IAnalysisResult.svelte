<script lang="ts">
	import ITip from './ITip.svelte';
	import type {Tip} from './types';

	export let file_path: string;
	export let tips: Tip[];
</script>

<div class="space-y-2" id="analysis-result">
	<!-- <p>{file_path}</p> -->
	{#each tips as tip}
		<ITip
			property_name={tip.property_name}
			property_value={tip.property_value}
			message={tip.message}
		/>
	{/each}
</div>
